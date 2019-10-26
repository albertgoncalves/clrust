with import <nixpkgs> {};
let
    shared = [
        (python37.withPackages(ps: with ps; [
            flake8
            matplotlib
            numpy
            pandas
            pytest
        ]))
        csvkit
        rustup
        shellcheck
    ];
    hook = ''
        . .shellhook
    '';
in
{
    darwin = mkShell {
        buildInputs = shared;
        shellHook = hook;
    };
    linux = mkShell {
        buildInputs = [
            pkg-config
        ] ++ shared;
        shellHook = hook;
    };
}
