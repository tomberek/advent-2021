{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    mozillapkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, mozillapkgs }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";

      # Get a specific rust version
      mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") {};
      rust = (mozilla.rustChannelOf {
        date = "2021-12-04"; # get the current date with `date -I`
        channel = "nightly";
        sha256 = "sha256-GnDd37tKPYOWzfGQb+TBdouv2R/NDbxZ3wGW/1o+Hy8=";
      }).rust;

      # Override the version used in naersk
      naersk-lib = naersk.lib."${system}".override {
        cargo = rust;
        rustc = rust;
      };
    in rec {
      # `nix build`
      packages.my-project = naersk-lib.buildPackage {
        pname = "my-project";
        root = ./.;
      };
      defaultPackage = packages.my-project;

      # `nix run`
      apps.my-project = utils.lib.mkApp {
        drv = packages.my-project;
      };
      defaultApp = apps.my-project;

      # `nix develop`
      devShell = pkgs.mkShell {
        # supply the specific rust version
        nativeBuildInputs = [ rust ];
        shellHook = ''
          export PS1='\[\033[1;33m\][\w]\$\[\033[0m\] '
          asm(){
            cargo asm --rust "$@"
          }

          '';
      };
    });
}
