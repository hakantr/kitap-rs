## Kodu Aynı Anda Çalıştırmak İçin İş Parçacıkları Kullanmak

Günümüzde çoğu işletim sisteminde çalışan program kodu bir _süreç (process)_
içinde yürütülür ve işletim sistemi aynı anda birden fazla süreci yönetir.
Bir programın içinde de aynı anda ilerleyen bağımsız parçalar olabilir. Bu
bağımsız parçaları çalıştıran yapılara _iş parçacığı (thread)_ denir. Örneğin
bir web sunucusu, aynı anda birden fazla isteğe yanıt verebilmek için birden
çok iş parçacığı kullanabilir.

Programınızdaki hesabı birden fazla iş parçacığına bölüp görevleri aynı anda
çalıştırmak performansı artırabilir; ama bunun karşılığında karmaşıklık da
artar. İş parçacıkları aynı anda çalışabildiği için, farklı iş parçacıklarında
yer alan kodunuzun hangi sırayla işleyeceğine dair doğal bir garanti yoktur.
Bu da şu tür sorunlara yol açabilir:

- İş parçacıklarının veri ya da kaynaklara tutarsız bir sırayla eriştiği yarış
  durumları
- İki iş parçacığının birbirini beklediği ve ikisinin de devam edemediği
  kilitlenmeler
- Yalnızca bazı durumlarda ortaya çıkan, tekrar üretmesi ve güvenle düzeltmesi
  zor hatalar

Rust, iş parçacığı kullanmanın olumsuz yanlarını azaltmaya çalışır; ama çok iş
parçacıklı programlama yine de dikkatli düşünmeyi ve tek iş parçacıklı
programlardan farklı bir kod yapısını gerektirir.

Programlama dilleri iş parçacıklarını çeşitli şekillerde uygular; birçok
işletim sistemi de yeni iş parçacığı oluşturmak için çağrılabilen bir API
sunar. Rust standart kütüphanesi iş parçacıkları için _1:1_ modelini kullanır:
Programdaki her dil düzeyi iş parçacığı için bir işletim sistemi iş parçacığı
vardır. Farklı ödünleşimler sunan başka iş parçacığı modellerini uygulayan
crate'ler de vardır. Bir sonraki bölümde göreceğimiz Rust'ın `async` sistemi de
eşzamanlılığa farklı bir yaklaşım sunar.

### `spawn` ile Yeni Bir İş Parçacığı Oluşturmak

Yeni bir iş parçacığı oluşturmak için `thread::spawn` fonksiyonunu çağırır ve
ona, yeni iş parçacığında çalıştırmak istediğimiz kodu içeren bir kapanış
veririz. 16-1 numaralı liste, ana iş parçacığından biraz metin; yeni oluşturulan
iş parçacığından da başka metinler yazdırır.

<Listing number="16-1" file-name="src/main.rs" caption="Ana iş parçacığı başka bir şey yazdırırken yeni bir iş parçacığı oluşturmak">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

</Listing>

Rust programında ana iş parçacığı tamamlandığında, oluşturulmuş bütün iş
parçacıkları işleri bitmiş olsun ya da olmasın kapatılır. Bu programın çıktısı
her çalıştırmada biraz farklı olabilir; ama aşağıdakine benzer görünür:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
ana iş parçacığından merhaba sayı 1!
oluşturulan iş parçacığından merhaba sayı 1!
ana iş parçacığından merhaba sayı 2!
oluşturulan iş parçacığından merhaba sayı 2!
ana iş parçacığından merhaba sayı 3!
oluşturulan iş parçacığından merhaba sayı 3!
ana iş parçacığından merhaba sayı 4!
oluşturulan iş parçacığından merhaba sayı 4!
oluşturulan iş parçacığından merhaba sayı 5!
```

`thread::sleep` çağrıları bir iş parçacığını kısa süreliğine durdurur ve başka
bir iş parçacığının çalışmasına fırsat verir. Çoğu zaman iş parçacıkları sıra
ile ilerler; ama bu garanti değildir. Her şey işletim sisteminizin iş
parçacıklarını nasıl zamanladığına bağlıdır. Bu çalıştırmada, kodda önce
oluşturulan iş parçacığındaki `println!` görünse de ilk yazdıran ana iş
parçacığı oldu. Ayrıca oluşturulan iş parçacığına `i` değeri `9` olana kadar
yazdırmasını söylemiş olsak da, ana iş parçacığı kapanmadan önce ancak `5`
sayısına kadar gelebildi.

Bu kodu çalıştırıp yalnızca ana iş parçacığının çıktısını görürseniz ya da hiç
örtüşme görmezseniz, aralıkları büyütmeyi deneyin. Böylece işletim sisteminin
iki iş parçacığı arasında geçiş yapması için daha fazla fırsat oluşur.

<!-- Old headings. Do not remove or links may break. -->

<a id="waiting-for-all-threads-to-finish-using-join-handles"></a>

### Tüm İş Parçacıklarının Bitmesini Beklemek

16-1 numaralı listedeki kodun sorunu şu: Ana iş parçacığı çoğu zaman daha erken
bittiği için oluşturulan iş parçacığı yarıda kesiliyor. Üstelik iş parçacıkları
hangi sırayla çalışacak belli olmadığından, oluşturulan iş parçacığının hiç
çalışacağı da garanti değil.

Bu sorunu çözmek için `thread::spawn` dönüş değerini bir değişkende
saklayabiliriz. `thread::spawn` fonksiyonu `JoinHandle<T>` döndürür.
`JoinHandle<T>`, sahip olunan bir değerdir; bunun üstünde `join` metodunu
çağırdığımızda ilgili iş parçacığının tamamlanmasını bekleriz. 16-2 numaralı
liste, 16-1'de oluşturduğumuz iş parçacığı için dönen `JoinHandle<T>` değerini
nasıl kullandığımızı ve `main` sonlanmadan önce bu iş parçacığının bitmesini
garanti etmek için `join` metodunu nasıl çağırdığımızı gösteriyor.

<Listing number="16-2" file-name="src/main.rs" caption="İş parçacığının sonuna kadar çalışmasını garanti etmek için `thread::spawn` dönüşündeki `JoinHandle<T>` değerini saklamak">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

</Listing>

`tutamac` üzerinde `join` çağırmak, şu anda çalışan iş parçacığını, tutamacın
temsil ettiği iş parçacığı bitene kadar _bloklar (blocking)_. Bir iş parçacığının
bloklanması, onun iş yapmasının ya da sonlanmasının geçici olarak engellenmesi
demektir. `join` çağrısını ana iş parçacığındaki `for` döngüsünden sonraya
koyduğumuz için, 16-2 numaralı listenin çıktısı şuna benzer:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
ana iş parçacığından merhaba sayı 1!
ana iş parçacığından merhaba sayı 2!
oluşturulan iş parçacığından merhaba sayı 1!
ana iş parçacığından merhaba sayı 3!
oluşturulan iş parçacığından merhaba sayı 2!
ana iş parçacığından merhaba sayı 4!
oluşturulan iş parçacığından merhaba sayı 3!
oluşturulan iş parçacığından merhaba sayı 4!
oluşturulan iş parçacığından merhaba sayı 5!
oluşturulan iş parçacığından merhaba sayı 6!
oluşturulan iş parçacığından merhaba sayı 7!
oluşturulan iş parçacığından merhaba sayı 8!
oluşturulan iş parçacığından merhaba sayı 9!
```

İki iş parçacığı dönüşümlü ilerlemeyi sürdürür; ama `tutamac.join()` çağrısı
nedeniyle ana iş parçacığı sona ermez ve oluşturulan iş parçacığının
tamamlanmasını bekler.

Şimdi de `tutamac.join()` çağrısını `main` içindeki `for` döngüsünden önceye
taşırsak ne olur, ona bakalım:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

</Listing>

Bu durumda ana iş parçacığı önce oluşturulan iş parçacığının bitmesini bekler,
sonra kendi `for` döngüsünü çalıştırır. Yani çıktılar artık iç içe geçmez:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
oluşturulan iş parçacığından merhaba sayı 1!
oluşturulan iş parçacığından merhaba sayı 2!
oluşturulan iş parçacığından merhaba sayı 3!
oluşturulan iş parçacığından merhaba sayı 4!
oluşturulan iş parçacığından merhaba sayı 5!
oluşturulan iş parçacığından merhaba sayı 6!
oluşturulan iş parçacığından merhaba sayı 7!
oluşturulan iş parçacığından merhaba sayı 8!
oluşturulan iş parçacığından merhaba sayı 9!
ana iş parçacığından merhaba sayı 1!
ana iş parçacığından merhaba sayı 2!
ana iş parçacığından merhaba sayı 3!
ana iş parçacığından merhaba sayı 4!
```

`join` çağrısının nereye yazıldığı gibi küçük ayrıntılar bile iş
parçacıklarının gerçekten aynı anda çalışıp çalışmayacağını etkileyebilir.

### İş Parçacıklarıyla `move` Kapanışları Kullanmak

`thread::spawn` içine verdiğimiz kapanışlarla birlikte çoğu zaman `move`
anahtar sözcüğünü de kullanırız. Çünkü bu durumda kapanış, kullandığı
değerlerin sahipliğini çevresinden alır; böylece o değerlerin sahipliği bir iş
parçacığından diğerine aktarılmış olur. 13. bölümde [“Referansları Yakalamak
ya da Sahipliği Taşımak”][capture]<!-- ignore --> kısmında `move` anahtar
sözcüğünü kapanışlar bağlamında görmüştük. Şimdi `move` ile `thread::spawn`
arasındaki ilişkiye odaklanacağız.

16-1 numaralı listedeki kapanışın hiç parametre almadığına dikkat edin: Ana iş
parçacığındaki hiçbir veriyi, oluşturulan iş parçacığındaki kodda
kullanmıyoruz. Oluşturulan iş parçacığında ana iş parçacığından veri
kullanabilmek için, kapanış ihtiyaç duyduğu değerleri yakalamalıdır. 16-3
numaralı liste, ana iş parçacığında bir vektör oluşturup onu oluşturulan iş
parçacığında kullanma girişimini gösteriyor. Ama birazdan göreceğiniz gibi bu
halde çalışmaz.

<Listing number="16-3" file-name="src/main.rs" caption="Ana iş parçacığında oluşturulan bir vektörü başka bir iş parçacığında kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

</Listing>

Kapanış `vektor` değişkenini kullandığı için onu yakalar ve kendi çevresinin
bir parçası haline getirir. `thread::spawn` bu kapanışı yeni bir iş
parçacığında çalıştırdığına göre, sanki bu yeni iş parçacığında `vektor`
değerine erişebilmemiz gerekiyormuş gibi görünür. Fakat bu örneği derlediğimizde
şu hatayı alırız:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

Rust, `vektor` değişkeninin nasıl yakalanacağını _çıkarsar (infer)_. Burada
`println!` yalnızca `vektor` için bir referansa ihtiyaç duyduğu için kapanış
onu ödünç almaya çalışır. Sorun şu ki Rust, oluşturulan iş parçacığının ne
kadar yaşayacağını bilemez; bu yüzden `vektor` için alınan referansın her zaman
geçerli kalıp kalmayacağını da bilemez.

16-4 numaralı liste, `vektor` referansının geçersiz hale gelmesinin çok daha
olası olduğu bir senaryo gösterir.

<Listing number="16-4" file-name="src/main.rs" caption="Ana iş parçacığı `vektor` değerini düşürürken, kapanış içinde ona referans yakalamaya çalışan bir iş parçacığı">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

</Listing>

Rust bu kodu çalıştırmamıza izin verseydi, oluşturulan iş parçacığı hiç
çalışmadan hemen arka plana alınabilirdi. O iş parçacığının içinde `vektor`e
ait bir referans var; ama ana iş parçacığı 15. bölümde gördüğümüz `drop`
fonksiyonunu kullanarak `vektor`ü anında serbest bırakıyor. Sonra oluşturulan
iş parçacığı çalışmaya başladığında `vektor` artık geçerli olmuyor; dolayısıyla
ona ait referans da geçersiz hale geliyor. Kötü haber!

16-3 numaralı listedeki derleyici hatasını düzeltmek için hata mesajındaki
öneriyi uygulayabiliriz:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `vektor` (and any other referenced variables), use the `move` keyword
  |
6 |     let tutamac = thread::spawn(move || {
  |                                ++++
```

Kapanışın önüne `move` eklediğimizde, Rust'ın ödünç alma çıkarımı yapmasına
izin vermek yerine kullanılan değerlerin sahipliğini kapanışın almasını
zorlamış oluruz. 16-3 numaralı listedeki örneğin 16-5'te gösterilen bu
değiştirilmiş hali, istediğimiz gibi derlenir ve çalışır.

<Listing number="16-5" file-name="src/main.rs" caption="Bir kapanışın kullandığı değerlerin sahipliğini almasını zorlamak için `move` kullanmak">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

</Listing>

16-4 numaralı listedeki kodu da aynı yolla düzeltmeyi düşünebiliriz. Orada ana
iş parçacığı `drop` çağrısıyla `vektor`ü düşürüyordu. Fakat 16-5'teki gibi
`move` kullandığımız anda, `vektor`ün sahipliği kapanışa aktarılır. Bu yüzden
ana iş parçacığında artık `drop(vektor)` çağırmak mümkün olmaz. Bunu
denediğimizde, bu kez aşağıdaki gibi farklı bir derleyici hatası alırız:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

Rust'ın sahiplik kuralları bizi yine korumuş oldu.

[capture]: ch13-01-kapanislar.html#referansları-yakalamak-capturing-veya-sahipliği-ownership-taşımak
