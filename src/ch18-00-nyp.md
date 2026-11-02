# Nesne Yönelimli Programlama Özellikleri

<!-- Old headings. Do not remove or links may break. -->

<a id="object-oriented-programming-features"></a>
<a id="object-oriented-programming-features-of-rust"></a>

Nesne yönelimli programlama (NYP; İng. _object-oriented programming_, _OOP_), programları modellemenin yollarından biridir. Bir programlama kavramı olarak nesneler, 1960'larda Simula programlama dilinde ortaya çıktı. Bu nesneler, Alan Kay'in nesnelerin birbirine mesaj ilettiği programlama mimarisini etkiledi. Kay, 1967'de bu mimariyi tanımlamak için _nesne yönelimli programlama_ terimini ortaya attı. NYP'nin ne olduğuna dair birbiriyle çelişen pek çok tanım vardır; bu tanımların bazılarına göre Rust nesne yönelimlidir, bazılarına göreyse değildir.

Bu bölümde, yaygın olarak nesne yönelimli kabul edilen bazı karakteristikleri ve bu karakteristiklerin deyimsel Rust'ta nasıl karşılık bulduğunu inceleyeceğiz. Ardından nesne yönelimli bir tasarım desenini Rust'ta nasıl uygulayacağınızı gösterecek, bu yolun ödünleşimlerini (*trade-offs*) de Rust'ın güçlü yönlerinden yararlanan bir çözümle karşılaştırarak tartışacağız.

> Not: Rust çok paradigmalı (*multi-paradigm*) bir dildir: veriyi `struct` ve
> `enum` ile, davranışı ise `impl` blokları ve `trait`'lerle modeller. Klasik
> sınıf tabanlı kalıtım (*class inheritance*) yerine **bileşim (*composition*)**,
> **trait sınırları (*trait bounds*)**, **trait nesneleri (`dyn Trait`)** ve
> **cebirsel veri türleri (`enum`)** kullanarak hem çalışma zamanı esnekliğini
> hem de derleme zamanı tür ve sahiplik güvenliğini bir arada sunar.
