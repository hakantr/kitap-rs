# Nesne Yönelimli Programlama Özellikleri

<!-- Old headings. Do not remove or links may break. -->

<a id="object-oriented-programming-features"></a>
<a id="object-oriented-programming-features-of-rust"></a>

Nesne yönelimli programlama (NYP; İng. _object-oriented programming_, _OOP_),
programları modellemenin yollarından biridir. Programatik bir kavram olarak
nesneler, 1960'larda Simula programlama dilinde ortaya çıktı. Bu nesneler, Alan
Kay'in nesnelerin birbirine mesaj ilettiği programlama mimarisini etkiledi. Kay,
1967'de bu mimariyi tanımlamak için _nesne yönelimli programlama
(object-oriented programming)_ terimini ortaya attı. NYP'nin ne olduğunu
açıklayan birbiriyle rekabet halinde pek çok tanım vardır ve bu tanımlardan
bazılarına göre Rust nesne yönelimlidir, bazılarına göreyse değildir.

Bu bölümde, yaygın biçimde nesne yönelimli kabul edilen belirli karakteristikleri
ve bu karakteristiklerin deyimsel Rust'a nasıl tercüme edildiğini inceleyeceğiz.
Ardından nesne yönelimli bir tasarım desenini Rust'ta nasıl uygulayacağınızı
gösterecek ve bunu yapmanın ödünleşimlerini, Rust'ın güçlü yönlerinden
yararlanan bir çözüm geliştirmekle karşılaştırarak tartışacağız.

> Not: Rust çok paradigmalı (*multi-paradigm*) bir dildir: veriyi `struct` ve
> `enum` ile, davranışı ise `impl` blokları ve `trait`'lerle modeller. Klasik
> sınıf tabanlı kalıtım (*class inheritance*) yerine **bileşim (*composition*)**,
> **trait sınırları (*trait bounds*)**, **trait nesneleri (`dyn Trait`)** ve
> **cebirsel veri türleri (`enum`)** kullanarak hem çalışma zamanı esnekliğini
> hem de derleme zamanı tür ve sahiplik güvenliğini bir arada sunar.
