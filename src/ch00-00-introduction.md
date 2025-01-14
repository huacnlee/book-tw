# 介紹

> 注意：本書的（英文）版本與出版的 [The Rust Programming Language][nsprust] 以及電子書版本的 [No Starch Press][nsp] 一致。

[nsprust]: https://nostarch.com/rust
[nsp]: https://nostarch.com/

歡迎閱讀 **Rust 程式設計語言**，這是本 Rust 的入門書籍。Rust 程式設計語言能幫助你寫出更快更可靠的軟體。高層的易讀易用性與底層的掌控性常常是程式設計語言之間的取捨，Rust 試圖挑戰這項矛盾。透過平衡強大的技術能力以及優秀的開發體驗，Rust 讓你能控制底層的實作細節（比如記憶體使用）的同時，免於以往這樣的控制所帶來的相關麻煩。

## Rust 適用於誰

Rust 有非常多種誘因使其適用於各式各樣的人。讓我們討論一些比較重要的目標客群。

### 開發團隊

Rust 已被證明是個能在各種系統程式設計領域的大型開發團隊中協作的高效開發工具。底層程式碼容易產生微妙的程式錯誤，這在多數其他語言只能靠密集的測試並由經驗豐富的開發者小心翼翼地審核程式碼才能找出它們。在 Rust 中，編譯器會扮演著守門員的角色來阻擋這些難以捉摸的程式錯誤，這包含了並行錯誤。透過與編譯器一同合作，開發團隊可以將他們的時間專注在程式邏輯而不必成天追蹤錯誤。

Rust 也將一些現代化的開發工具帶入系統程式設計的世界中：

* Cargo 是個管理依賴函式庫暨建構的工具，讓新增、編譯與管理依賴變得十分輕鬆，並在 Rust 生態系統維持一致性。
* Rustfmt 工具確保開發者遵循統一的程式碼風格。
* Rust Language Server 為整合開發環境（IDE）提供了程式碼補全與行內錯誤訊息。

透過使用這些與其他 Rust 生態系統中的工具，開發者可以在寫系統層級的程式語言時，維持一定的生產力。

### 學生

Rust 適用於學生以及對學習系統概念有興趣的人。許多人都透過 Rust 來學習作業系統開發的議題。社群的人都非常友善，且樂於解答學生們的問題。除了本書以外，Rust 團隊也希望系統概念可以被更多人理解，尤其是剛開始學習程式設計的人。

### 公司

已有大大小小數以百計的公司，在生產環境使用 Rust 來處理各種任務。包含命令列工具、網路服務、DevOps 工具、嵌入式裝置、影音分析與轉碼、加密貨幣、生物資訊、搜尋引擎、物聯網裝置、機器學習，甚至還有 Firefox 瀏覽器的主要部分。

### 開源開發者

Rust 適用於想要建構 Rust 程式設計語言、社群、開發工具與函式庫的開發者。我們很樂於看到你願意對 Rust 語言貢獻。

### 重視速度與穩定性的開發者

Rust 適用於追求語言速度與穩定性的開發者。所謂的速度，我們指的是 Rust 程式碼的執行速度以及 Rust 提供你開發程式的速度。Rust 編譯器的檢查能確保新增功能與重構時的穩定性。這與沒有這些檢查的語言形成對比，開發者通常會害怕修改這些脆弱的遺留程式碼。Rust 還力求零開銷抽象（zero-cost abstractions），高階的特性編譯成底層程式碼的執行速度能與自己手寫一樣快，Rust 致力於讓安全的程式碼同時也能是執行迅速的程式碼。

Rust 語言也希望能支援其他許多使用者，這裡提及的只是一部分的最大受益者。總體來說，Rust 最重要的目標是消除數十年來開發者不得不作出的取捨，像是同時提供安全**與**生產力、同時具有速度**又**易讀易用。歡迎嘗試看看 Rust，並體驗看看這門語言適不適合你。

## 本書寫給誰看

本書假設你已經使用其他程式語言寫過程式碼，不過並不假設你使用的是何種語言。我們試著讓這些資源能廣泛適用於不同程式背景的開發者。我們不會花費很多時間討論**什麼是**程式設計或如何理解它。如果你剛開始學習程式語言，你最好先閱讀專門介紹程式設計的書籍。

## 如何閱讀本書

一般來說，本書預設你會從頭到尾依序閱讀完。後面的章節都建立在前面章節的概念基礎上，然後前面的章節可能不會深入探討特定主題，不過會在後面的章節再重新討論該主題。

你會在本書中看到兩種章節：概念章節與專案章節。在概念章節中，你會學到 Rust 的其中某些概念。在專案章節中，我們會應用你當前所學的一同建構些小小的專案。第二、十二和二十章是專案章節，其餘都是概念章節。

第一章會解釋如何安裝 Rust、如何寫支「Hello, world!」程式以及如何使用 Cargo－－Rust 的套件與建構工具。第二章是透過實作一款猜謎遊戲的程式來介紹 Rust 的章節。我們在此會以較高階的方式解釋些概念，並在之後的章節提供更詳細的介紹。如果你想馬上動手實作看看的話，第二章會很適合你。第三章會涵蓋 Rust 與其他程式設計語言類似的功能。第四章則會學習 Rust 的所有權系統。如果你是那種鉅細靡遺的讀者，傾向於在進行下一步前學習到所有細節的話，你可能會想跳過第二章直接前往第三章，當你想要套用你所學到的細節在專案上時，再回頭到第二章練習。

第五章討論結構體與方法，而第六章涵蓋枚舉、`match` 表達式與 `if let` 控制流結構。你會在 Rust 中用結構體與枚舉來自訂型別。

在第七章中，你會學到 Rust 的模組系統與組織程式碼的隱私規則，以及其公開應用程式介面（Application Programming Interface, API）。第八章會討論標準函式庫提供的一些常見集合資料結構，像是向量、字串與雜湊映射。第九章會探討 Rust 的錯誤處理哲學與技巧。

第十章將深入探討泛型、特徵與生命週期，讓你能定義出多種型別的程式碼。第十一章全部都關於測試，這與 Rust 的安全保障一樣重要，以確保程式邏輯正確。在第十二章中，我們會親自實作個 `grep` 命令列工具的子集功能，來搜尋檔案中的文字。我們會利用到前幾章討論過的許多概念。

第十三章會探索閉包與疊代器，這是 Rust 啟發自函式程式設計語言的功能。在第十四章中，我們要更深入研究 Cargo 並討論分享函式庫給其他人的最佳方式。第十五章會討論標準函式庫提供的智慧指標以及能啟用它們功能的特徵。

在第十六章中，我們會介紹各種不同的並行程式設計模型，並介紹 Rust 如何幫助你在多執行緒中無懼地開發。第十七章會比較 Rust 的慣用風格與你可能較熟悉的物件導向程式設計原則之間有何差異。

第十八章是模式與模式配對的參考章節，這是在 Rust 程式中表達不同條件的強大特色。第十九章是進階主題的大雜燴，其中包含不安全的 Rust、巨集與更多有關生命週期、特徵、型別、函式與閉包的資訊。

在第二十章中，我們會完成一項專案，那就是實作底層多執行緒的網頁伺服器！

最後的附錄以參考風格格式來包含語言中的實用資訊。附錄 A 涵蓋 Rust 的關鍵字、附錄 B 涵蓋 Rust 的運算子與符號、附錄 C 涵蓋標準函式庫提供的可推導的特徵、附錄 D 涵蓋一些實用開發工具，然後附錄 E 會解釋 Rust 的版號。在附錄 F 中你可以找到本書籍的各種翻譯版本，然後在附錄 G 我們會講解 Rust 的開發流程以及什麼是夜版 Rust。

本書沒有錯誤的閱讀方式：如果你想要忽略一些章節，儘管跳過吧！如果你遇到任何疑惑的話，隨時可以再跳回之前的章節閱讀。請隨意閱讀。

<span id="ferris"></span>

學習 Rust 的過程中有個重要的部分就是要學習如何閱讀編譯器顯示的錯誤訊息，這些能引導你寫出正確的程式碼。所以我們會提供很多無法編譯的範例，以及各種編譯器會顯示訊息的狀況。如果你執行任何範例的話，它很可能不會編譯通過！別忘了看看周遭的文字來瞭解該執行範例是不是故意出錯。Ferris 也會來幫助你分辨哪些程式碼本來就無法使用：

| Ferris                                                                                                           | Meaning                                          |
|------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | 此程式碼無法編譯！                      |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | 此程式碼會恐慌！                                |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | 此程式碼沒有產生預期的行為。 |

在大多數的情況下，我們會引導你將無法編譯的程式碼寫成正確的版本。

## 原始碼

產生本書的原始檔案可以在 [GitHub][book] 上找到。

[book]: https://github.com/rust-tw/book-tw
