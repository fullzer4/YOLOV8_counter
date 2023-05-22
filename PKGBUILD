pkgname=spotify-cli
pkgver=1.0.0
pkgrel=1
pkgdesc="Cliente de linha de comando para o Spotify para tocar musicas"
arch=('x86_64')
url="https://github.com/fullzer4/SpotifyCLI"
license=('MIT')
depends=('rust')
makedepends=('cargo')
source=("file://$PWD/package/spotify-cli.tar.gz")
sha256sums=('6a3098f88062ca3bf93fe2bb303e14a27122eb56e9312224cab147ecd814e137')

build() {
  cd "../spotify_cli"
  cargo build --release --locked
}

package() {
  pwd
  install -Dm755 "../spotify_cli/target/release/spotify_cli" "$pkgdir/usr/bin/spotify-cli"
}