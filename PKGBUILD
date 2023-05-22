pkgname=spotify-cli
pkgver=1.0.0
pkgrel=1
pkgdesc="Cliente de linha de comando para o Spotify para tocar musicas"
arch=('x86_64')
url="https://github.com/seu-usuario/spotify-cli"
license=('MIT')
depends=('rust')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/fullzer4/SpotifyCLI_CS50Final_project/blob/bf4d9e5ccb5c778e791602621b05649f22d98b1b/package/spotify-cli.tar.gz")
sha256sums=('0f0e4c5b5ebf38f0d6b56da6fce82ee732a53f5f88c17e3172fe0006737331ca')

build() {
  cd "../spotify_cli"
  cargo build --release --locked
}

package() {
  pwd
  install -Dm755 "../spotify_cli/target/release/spotify_cli" "$pkgdir/usr/bin/spotify-cli"
}