<!-- Old headings. Do not remove or links may break. -->
<a id="understanding-ownership"></a>

# Sahipliği Anlamak

*Sahiplik (ownership)*, Rust'ın en ayırt edici özelliğidir ve dilin geri kalanı üzerinde derin etkilere sahiptir. Rust'ın bir *çöp toplayıcıya (garbage collector)* ihtiyaç duymadan bellek güvenliği garantisi sunmasını sağlar; bu nedenle sahipliğin nasıl çalıştığını anlamak büyük önem taşır. Bu bölümde sahipliğin yanı sıra onunla yakından ilişkili birkaç özelliği daha ele alacağız: *ödünç alma (borrowing)*, *dilimler (slices)* ve Rust'ın verileri bellekte nasıl yerleştirdiği.

> Not: Bu bölüm, Rust'ı diğer programlama dillerinden ayıran en temel tasarım felsefesine giriş niteliğindedir. C ve C++ gibi dillerde bellek yönetimi doğrudan geliştiricinin sorumluluğundayken, Java, Python veya Go gibi dillerde çalışma zamanında devreye giren bir çöp toplayıcı kullanılır. Rust ise bellek güvenliğini derleme zamanında, çalışma zamanında ek yük oluşturmadan denetleyen *ödünç alma denetleyicisi (borrow checker)* ile üçüncü bir yol açar. İlk başta bu kurallar alışılmadık gelebilir; ancak karşılaşacağınız her derleyici hatası, çalışma zamanında ortaya çıkabilecek zorlu bir bellek hatasını henüz kodunuz derlenirken yakalayan bir güvenlik ağıdır. Bu bölümdeki *yığın (stack)*, *öbek (heap)*, sahiplik ve referans kavramlarını sindirerek ilerlemek, kitabın geri kalanındaki tüm konuları çok daha rahat kavramanızı sağlayacaktır.
