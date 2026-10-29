<!-- Old headings. Do not remove or links may break. -->
<a id="writing-automated-tests"></a>
<a id="otomatik-testler-yazmak"></a>

# Otomatik Testler Yazma

Edsger W. Dijkstra, 1972 tarihli "Mütevazı Programcı" adlı makalesinde şöyle demiştir: "Programları test etmek, hataların varlığını göstermek için çok etkili bir yol olabilir; ancak hataların yokluğunu göstermek konusunda umutsuzca yetersizdir." Elbette bu, elimizden geldiğince kapsamlı test yapmaya çalışmamamız gerektiği anlamına gelmez!

Programlarımızdaki _doğruluk_, kodumuzun amaçladığımız işi ne ölçüde yerine getirdiğidir. Rust, programların doğruluğuna büyük özen gösterilerek tasarlanmıştır; ancak doğruluk karmaşık bir kavramdır ve kanıtlanması kolay değildir. Rust'ın tür sistemi bu yükün çok büyük bir kısmını üstlense de her hatayı tek başına yakalayamaz. Bu nedenle Rust, otomatik yazılım testleri yazma desteğini doğrudan bünyesinde barındırır.

Diyelim ki kendisine geçirilen sayıya 2 ekleyen `iki_ekle` adında bir fonksiyon yazdık. Bu fonksiyonun imzası, parametre olarak bir tam sayı alır ve sonuç olarak yine bir tam sayı döndürür. Fonksiyonu yazıp derlediğimizde Rust, tür sistemi ve ödünç alma denetleyicisi aracılığıyla şimdiye kadar öğrendiğiniz tüm denetimleri gerçekleştirir; böylece örneğin bu fonksiyona yanlışlıkla bir `String` değeri veya geçersiz bir referans geçirmediğimizden emin olur. Ne var ki Rust, bu fonksiyonun tam olarak bizim amaçladığımız işlemi yapıp yapmadığını — söz gelimi parametreye 10 eklemek ya da parametreden 50 çıkarmak yerine gerçekten 2 ekleyip döndürdüğünü — kontrol _edemez_! İşte testler tam bu noktada devreye girer.

Örneğin `iki_ekle` fonksiyonuna `3` geçirdiğimizde dönen değerin `5` olduğunu doğrulayan testler yazabiliriz. Kodumuzda ne zaman bir değişiklik yapsak, mevcut doğru davranışın değişmediğinden emin olmak için bu testleri çalıştırabiliriz.

Test yazmak karmaşık bir beceridir: iyi testlerin nasıl yazılacağına dair her ayrıntıyı tek bir bölümde ele almamız mümkün olmasa da, bu bölümde Rust'ın test olanaklarının nasıl çalıştığını inceleyeceğiz. Testlerinizi yazarken kullanabileceğiniz öznitelikleri ve makroları, testleri çalıştırmak için sunulan varsayılan davranışları ve seçenekleri, ayrıca testleri birim testleri ve entegrasyon testleri şeklinde nasıl düzenleyeceğinizi ele alacağız.

> Not: Pek çok programlama dilinde test yazmaya başlamadan önce üçüncü parti bir test çerçevesi seçmek, kurmak ve yapılandırmak gerekir. Rust'ta ise test altyapısı doğrudan dile ve Cargo'ya yerleşiktir: `#[test]` ve `#[should_panic]` gibi öznitelikler ile `assert!`, `assert_eq!` ve `assert_ne!` doğrulama makroları standart kurulumla hazır gelir; `cargo test` komutu ise hiçbir ek paket kurmadan birim testlerini, entegrasyon testlerini ve hatta API belgelendirmesindeki kod örneklerini (*belgelendirme testleri*) tek komutla derleyip paralel olarak çalıştırır. Böylece derleyicinin tür sistemi ve ödünç alma denetleyicisiyle derleme zamanında sağladığı yapısal güvenliği, iş mantığınızı doğrulayan standart bir test akışıyla zahmetsizce tamamlayabilirsiniz.
