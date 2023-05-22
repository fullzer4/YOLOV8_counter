pkgname=spotify-cli
pkgver=1.0.0
pkgrel=1
pkgdesc="Cliente de linha de comando para o Spotify para tocar musicas"
arch=('x86_64')
url="https://github.com/seu-usuario/spotify-cli"
license=('MIT')
depends=('rust')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/seu-usuario/spotify-cli/archive/v$pkgver.tar.gz")
sha256sums=('xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 "target/release/spotify-cli" "$pkgdir/usr/bin/spotify-cli"
}