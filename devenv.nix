{
  pkgs,
  ...
}:

{
  packages = [
    pkgs.bashInteractive
    pkgs.cargo-audit
    pkgs.cargo-deny
    pkgs.go-task
    pkgs.shfmt
  ];

  languages = {
    rust = {
      enable = true;

      toolchainFile = ./rust-toolchain.toml;
    };
  };

  git-hooks = {
    hooks = {
      clippy = {
        enable = true;
        settings = {
          allFeatures = true;
        };
      };

      rustfmt = {
        enable = true;
      };
    };
  };
}
