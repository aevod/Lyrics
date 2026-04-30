{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
	packages = [ 
		pkgs.cargo
		pkgs.rustc
		pkgs.pkg-config
		pkgs.dbus
	];
}
