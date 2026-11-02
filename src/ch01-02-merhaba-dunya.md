<!-- Old headings. Do not remove or links may break. -->
<a id="hello-world"></a>

## Merhaba, Dünya!

Artık Rust'ı kurduğunuza göre ilk Rust programınızı yazmanın zamanı geldi. Yeni bir dil öğrenirken ekrana `Merhaba, dünya!` metnini yazdıran küçük bir program yazmak gelenektir; biz de burada aynısını yapacağız!

> Not: Bu kitap, komut satırına temel düzeyde aşina olduğunuzu varsayar. Rust; kullandığınız metin düzenleyici, araçlar veya kodunuzun nerede bulunduğu konusunda özel bir koşul dayatmaz. Bu nedenle komut satırı yerine tümleşik bir geliştirme ortamı (IDE) kullanmayı tercih ederseniz dilediğiniz IDE'yi rahatlıkla kullanabilirsiniz. Günümüzde pek çok IDE belirli düzeyde Rust desteği sunar; ayrıntılar için IDE'nin kendi belgelerini inceleyebilirsiniz. Rust ekibi, `rust-analyzer` aracılığıyla güçlü bir IDE desteği sağlamaya odaklanmıştır. Daha fazla ayrıntı için [Ek D][devtools]<!-- ignore --> bölümüne bakabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-project-directory"></a>
<a id="project-directory-setup"></a>

### Proje Dizinini Ayarlamak

İşe Rust kodunuzu saklayacağınız bir dizin oluşturarak başlayacaksınız. Kodunuzun nerede durduğu Rust açısından fark etmez; ancak bu kitaptaki alıştırmalar ve projeler için ev dizininizde bir _projects_ dizini oluşturmanızı ve tüm projelerinizi orada tutmanızı öneririz.

Bir terminal açın; _projects_ dizinini ve onun içinde "Merhaba, dünya!" projesinin dizinini oluşturmak için aşağıdaki komutları girin.

Linux, macOS ve Windows'taki PowerShell için şunu girin:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir merhaba_dunya
$ cd merhaba_dunya
```

Windows CMD için şunu girin:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir merhaba_dunya
> cd merhaba_dunya
```

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-and-running-a-rust-program"></a>
<a id="rust-program-basics"></a>

### Rust Programlamanın Temelleri

Ardından yeni bir kaynak dosya oluşturun ve adını _main.rs_ koyun. Rust kaynak dosyaları her zaman _.rs_ uzantısıyla biter. Dosya adınızda birden fazla kelime kullanıyorsanız kelimeleri ayırmak için alt çizgi (`_`) kullanmak genel kuraldır. Örneğin _merhabadunya.rs_ yerine _merhaba_dunya.rs_ kullanın.

Şimdi yeni oluşturduğunuz _main.rs_ dosyasını açın ve Liste 1-1'deki kodu girin.

<Listing number="1-1" file-name="main.rs" caption="Ekrana `Merhaba, dünya!` yazdıran bir program">

```rust
fn main() {
    println!("Merhaba, dünya!");
}
```

</Listing>

Dosyayı kaydedin ve _~/projects/merhaba_dunya_ dizinindeki terminal pencerenize geri dönün. Linux veya macOS üzerinde dosyayı derlemek ve çalıştırmak için aşağıdaki komutları girin:

```console
$ rustc main.rs
$ ./main
Merhaba, dünya!
```

Windows'ta `./main` yerine `.\main` komutunu girin:

```powershell
> rustc main.rs
> .\main
Merhaba, dünya!
```

İşletim sisteminiz ne olursa olsun terminale `Merhaba, dünya!` metni yazdırılmalıdır. Bu çıktıyı göremiyorsanız yardım alma yolları için Kurulum bölümündeki ["Sorun Giderme"][troubleshooting]<!-- ignore --> kısmına tekrar göz atın.

Eğer `Merhaba, dünya!` yazdırıldıysa tebrikler! Resmen bir Rust programı yazdınız. Bu sizi bir Rust programcısı yapar; aramıza hoş geldiniz!

<!-- Old headings. Do not remove or links may break. -->

<a id="anatomy-of-a-rust-program"></a>
<a id="the-anatomy-of-a-rust-program"></a>

### Bir Rust Programının Anatomisi

Şimdi bu "Merhaba, dünya!" programını ayrıntılı olarak inceleyelim. İşte yapbozun ilk parçası:

```rust
fn main() {

}
```

Bu satırlar `main` adında bir fonksiyon tanımlar. `main` fonksiyonu özeldir: Çalıştırılabilir her Rust programında çalışan ilk kod her zaman budur. Buradaki ilk satır, hiçbir parametre almayan ve değer döndürmeyen `main` adında bir fonksiyon bildirir. Parametreleri olsaydı bunlar parantezlerin (`()`) içine yazılırdı.

Fonksiyon gövdesi `{}` içine alınmıştır. Rust, tüm fonksiyon gövdelerinin süslü parantez içine alınmasını zorunlu kılar. Açılış süslü parantezini fonksiyon bildirimiyle aynı satıra koymak ve araya bir boşluk bırakmak iyi bir kodlama stili kabul edilir.

> Not: Rust projeleri genelinde standart bir kod stiline bağlı kalmak isterseniz kodunuzu belirli kurallara göre otomatik biçimlendiren `rustfmt` aracını kullanabilirsiniz (`rustfmt` hakkında daha fazla bilgi için [Ek D][devtools]<!-- ignore --> bölümüne bakın). Rust ekibi bu aracı tıpkı `rustc` gibi standart Rust dağıtımına dahil etmiştir; dolayısıyla bilgisayarınızda hâlihazırda kurulu olmalıdır!

`main` fonksiyonunun gövdesi şu kodu barındırır:

```rust
println!("Merhaba, dünya!");
```

Bu küçük programdaki tüm işi bu satır yapar: Ekrana metin yazdırır. Burada dikkat edilmesi gereken üç önemli ayrıntı vardır.

Birincisi, `println!` bir Rust makrosunu çağırır. Bunun yerine bir fonksiyon çağırsaydı `println` şeklinde (`!` olmadan) yazılırdı. Rust makroları, Rust sözdizimini genişletmek amacıyla kod üreten kodlar yazmanın bir yoludur; makroları [Bölüm 20'de][ch20-macros]<!-- ignore --> daha ayrıntılı ele alacağız. Şimdilik `!` kullanımının normal bir fonksiyon yerine bir makro çağırdığınız anlamına geldiğini ve makroların her zaman fonksiyonlarla aynı kurallara uymadığını bilmeniz yeterlidir.

İkincisi, `"Merhaba, dünya!"` metnini (*string*) görüyorsunuz. Bu metni `println!` makrosuna argüman olarak iletiyoruz ve metin ekrana yazdırılıyor.

Üçüncüsü, bu ifadenin sona erdiğini ve bir sonrakinin başlamaya hazır olduğunu belirten noktalı virgül (`;`) ile satırı sonlandırıyoruz. Rust kod satırlarının çoğu noktalı virgülle biter.

> Not: Rust'ta kaynak kod dosyaları ve tüm metin türleri varsayılan olarak **UTF-8** kodlamasını kullanır; bu sayede `"Merhaba, dünya!"` örneğindeki gibi Türkçe karakterleri (`ç, ğ, ı, İ, ö, ş, ü`) ek bir ayar yapmadan metinlerin içinde doğrudan kullanabilirsiniz. Buna karşılık dosya, değişken ve fonksiyon adlarında ekosistem uyumluluğu için Türkçe karakter içermeyen küçük harfli `snake_case` biçimi (`merhaba_dunya` gibi) tercih edilir. Ayrıca ilk kodunuzu yazarken `!` veya `;` gibi bir işareti unutursanız endişelenmeyin; `rustc` derleyicisi hatanın tam satır ve sütununu göstermenin yanı sıra doğrudan düzeltme ipucu da sunar (örneğin `println!` yerine `println` yazdığınızda ``help: use `!` to invoke the macro`` önerisiyle sizi yönlendirir).

<!-- Old headings. Do not remove or links may break. -->
<a id="compiling-and-running-are-separate-steps"></a>
<a id="compilation-and-execution"></a>

### Derleme ve Çalıştırma

Az önce yeni oluşturduğunuz bir programı çalıştırdınız; şimdi bu süreçteki her bir adımı inceleyelim.

Bir Rust programını çalıştırmadan önce, `rustc` komutuna kaynak dosyanızın adını vererek onu Rust derleyicisiyle derlemeniz gerekir:

```console
$ rustc main.rs
```

C veya C++ geçmişiniz varsa bunun `gcc` ya da `clang` kullanımına benzediğini fark edeceksiniz. Derleme başarıyla tamamlandığında Rust, *çalıştırılabilir bir ikili dosya (binary executable)* üretir.

Linux, macOS ve Windows'taki PowerShell'de, komut kabuğunuza `ls` komutunu girerek çalıştırılabilir dosyayı görebilirsiniz:

```console
$ ls
main  main.rs
```

Linux ve macOS'te iki dosya görürsünüz. Windows'ta PowerShell kullandığınızda ise CMD'dekiyle aynı üç dosyayı görürsünüz. Windows CMD'de şu komutu girebilirsiniz:

```cmd
> dir /B %= /B seçeneği yalnızca dosya adlarını gösterir =%
main.exe
main.pdb
main.rs
```

Bu çıktı; _.rs_ uzantılı kaynak kod dosyasını, çalıştırılabilir dosyayı (Windows'ta _main.exe_, diğer tüm platformlarda _main_) ve Windows kullanıyorsanız hata ayıklama bilgilerini içeren _.pdb_ uzantılı dosyayı gösterir. Buradan _main_ veya _main.exe_ dosyasını şu şekilde çalıştırabilirsiniz:

```console
$ ./main # veya Windows'ta .\main
```

Eğer _main.rs_ dosyanız "Merhaba, dünya!" programınızsa bu satır terminalinize `Merhaba, dünya!` yazdırır.

> Not: Linux, macOS ve PowerShell güvenlik nedeniyle bulunduğunuz dizindeki çalıştırılabilir dosyaları doğrudan adıyla (`main` yazarak) aramaz; yalnızca sistemin `PATH` ortam değişkenindeki dizinlere bakar. Baştaki `./` (veya Windows'ta `.\`) öneki, kabuğa programı **içinde bulunduğunuz dizinden** çalıştırmak istediğinizi açıkça belirtir.

Ruby, Python veya JavaScript gibi dinamik bir dile daha aşinaysanız bir programı derleme ve çalıştırma işlemlerini ayrı adımlar hâlinde yapmaya alışkın olmayabilirsiniz. Rust, *önceden derlenen (ahead-of-time compiled / AOT)* bir dildir; yani bir programı derleyip ürettiğiniz çalıştırılabilir dosyayı başka birine verebilirsiniz ve o kişi bilgisayarında Rust kurulu olmasa bile programı çalıştırabilir. Birine bir _.rb_, _.py_ veya _.js_ dosyası verirseniz karşı tarafın bilgisayarında sırasıyla bir Ruby, Python veya JavaScript dil uygulamasının (yorumlayıcısının veya çalışma ortamının) kurulu olması gerekir. Buna karşılık o dillerde programınızı derleyip çalıştırmak için tek bir komut yeterlidir. Dil tasarımında her karar bir ödünleşimdir.

Basit programlar için doğrudan `rustc` ile derleme yapmak yeterlidir; ancak projeniz büyüdükçe tüm derleme seçeneklerini yönetmek ve kodunuzu paylaşmayı kolaylaştırmak isteyeceksiniz. Bir sonraki bölümde, gerçek dünya Rust programları yazmanıza yardımcı olacak Cargo aracını tanıtacağız.

[troubleshooting]: ch01-01-kurulum.html#sorun-giderme
[devtools]: ekler-04-faydali-gelistirme-araclari.html
[ch20-macros]: ch20-05-makrolar.html
