{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    mozillapkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };

    dream2nix = {
      # url = "github:davhau/dream2nix";
      url = "github:yusdacra/dream2nix/fix/build-rust-package";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, naersk, mozillapkgs, dream2nix }@inputs:
    utils.lib.eachSystem ["x86_64-linux"] (system: let
      pkgs = nixpkgs.legacyPackages."${system}";

      # Get a specific rust version
      mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") {};
      rust = (mozilla.rustChannelOf {
        date = "2021-12-08"; # get the current date with `date -I`
        channel = "nightly";
        sha256 = "sha256-bJluL4R4+QCu4grw4/+4hP8r7NUvi07LhD9rkn4GTfg=";
      }).rust;

      # Override the version used in naersk
      naersk-lib = naersk.lib."${system}".override {
        cargo = rust;
        rustc = rust;
      };
      # System types to support.
      supportedSystems = [ "x86_64-linux" ];

      # Helper function to generate an attrset '{ x86_64-linux = f "x86_64-linux"; ... }'.
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      # Nixpkgs instantiated for supported system types.
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; 
        # overlays = [ self.overlay ];
      });


      dream2nix = inputs.dream2nix.lib.init {
        systems = supportedSystems;
        # config = {
        #   # overridesDirs = [ "${inputs.dream2nix}/overrides" ];
        # };
      };

    in rec {
      # `nix build`
      packages.my-project = naersk-lib.buildPackage {
        pname = "my-project";
        root = ./.;
      };
      # defaultPackage = packages.my-project;
      defaultPackage = (dream2nix.riseAndShine {
        source = builtins.path {
          name = "source";
          path = ./.;
        };
      }).defaultPackage.${system};

      checks = with nixpkgsFor.${system}; {
        all = runCommand "all" {buildInputs = [strace];} ''
          cp -r ${self}/.aocf .
          chmod +w .aocf
          ls -alh
          touch .aocf/cookie
          touch .aocf/config
          strace -f ${defaultPackage}/bin/day01
        '';
      };

      # `nix run`
      apps.my-project = utils.lib.mkApp {
        drv = packages.my-project;
      };
      defaultApp = apps.my-project;

      # `nix develop`
      devShell = pkgs.mkShell {
        # supply the specific rust version
        nativeBuildInputs = [ rust ];
        buildInputs = [ pkgs.pkg-config pkgs.openssl ];
        shellHook = ''
          export PS1='\[\033[1;33m\][\w]\$\[\033[0m\] '
          asm(){
            cargo asm --rust "$@"
          }

          '';
      };
    });
}
