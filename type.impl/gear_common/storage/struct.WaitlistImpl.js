(function() {var type_impls = {
"gear_common":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-CountedByKey-for-WaitlistImpl%3CT,+Value,+BlockNumber,+Error,+OutputError,+Callbacks,+KeyGen%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#178-193\">source</a><a href=\"#impl-CountedByKey-for-WaitlistImpl%3CT,+Value,+BlockNumber,+Error,+OutputError,+Callbacks,+KeyGen%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, Value, BlockNumber, Error, OutputError, Callbacks, KeyGen&gt; <a class=\"trait\" href=\"gear_common/storage/trait.CountedByKey.html\" title=\"trait gear_common::storage::CountedByKey\">CountedByKey</a> for <a class=\"struct\" href=\"gear_common/storage/struct.WaitlistImpl.html\" title=\"struct gear_common::storage::WaitlistImpl\">WaitlistImpl</a>&lt;T, Value, BlockNumber, Error, OutputError, Callbacks, KeyGen&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&lt;Value = (Value, <a class=\"struct\" href=\"gear_common/storage/struct.Interval.html\" title=\"struct gear_common::storage::Interval\">Interval</a>&lt;BlockNumber&gt;)&gt; + <a class=\"trait\" href=\"gear_common/storage/trait.CountedByKey.html\" title=\"trait gear_common::storage::CountedByKey\">CountedByKey</a>&lt;Key = T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key1\" title=\"type gear_common::storage::DoubleMapStorage::Key1\">Key1</a>&gt;,\n    Error: <a class=\"trait\" href=\"gear_common/storage/trait.WaitlistError.html\" title=\"trait gear_common::storage::WaitlistError\">WaitlistError</a>,\n    OutputError: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt;,\n    Callbacks: <a class=\"trait\" href=\"gear_common/storage/trait.WaitlistCallbacks.html\" title=\"trait gear_common::storage::WaitlistCallbacks\">WaitlistCallbacks</a>&lt;Value = Value, BlockNumber = BlockNumber&gt;,\n    KeyGen: <a class=\"trait\" href=\"gear_common/storage/trait.KeyFor.html\" title=\"trait gear_common::storage::KeyFor\">KeyFor</a>&lt;Key = (T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key1\" title=\"type gear_common::storage::DoubleMapStorage::Key1\">Key1</a>, T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key2\" title=\"type gear_common::storage::DoubleMapStorage::Key2\">Key2</a>), Value = Value&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Key\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Key\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.CountedByKey.html#associatedtype.Key\" class=\"associatedtype\">Key</a> = &lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key1\" title=\"type gear_common::storage::DoubleMapStorage::Key1\">Key1</a></h4></section></summary><div class='docblock'>Key type of counting target.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Length\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Length\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.CountedByKey.html#associatedtype.Length\" class=\"associatedtype\">Length</a> = &lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.CountedByKey.html\" title=\"trait gear_common::storage::CountedByKey\">CountedByKey</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.CountedByKey.html#associatedtype.Length\" title=\"type gear_common::storage::CountedByKey::Length\">Length</a></h4></section></summary><div class='docblock'>Returning length type.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.len\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#190-192\">source</a><a href=\"#method.len\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"gear_common/storage/trait.CountedByKey.html#tymethod.len\" class=\"fn\">len</a>(key: &amp;Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.CountedByKey.html#associatedtype.Key\" title=\"type gear_common::storage::CountedByKey::Key\">Key</a>) -&gt; Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.CountedByKey.html#associatedtype.Length\" title=\"type gear_common::storage::CountedByKey::Length\">Length</a></h4></section></summary><div class='docblock'>Returns current Self’s amount of elements as <code>Length</code> type.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_empty\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/primitives/counted.rs.html#56-58\">source</a><a href=\"#method.is_empty\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"gear_common/storage/trait.CountedByKey.html#method.is_empty\" class=\"fn\">is_empty</a>(key: &amp;Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.CountedByKey.html#associatedtype.Key\" title=\"type gear_common::storage::CountedByKey::Key\">Key</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Returns bool, defining if Self doesn’t contain elements.</div></details></div></details>","CountedByKey","gear_common::auxiliary::waitlist::AuxiliaryWaitlist"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IterableByKeyMap%3C%3CT+as+DoubleMapStorage%3E::Value%3E-for-WaitlistImpl%3CT,+Value,+BlockNumber,+Error,+OutputError,+Callbacks,+KeyGen%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#197-218\">source</a><a href=\"#impl-IterableByKeyMap%3C%3CT+as+DoubleMapStorage%3E::Value%3E-for-WaitlistImpl%3CT,+Value,+BlockNumber,+Error,+OutputError,+Callbacks,+KeyGen%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, Value, BlockNumber, Error, OutputError, Callbacks, KeyGen&gt; <a class=\"trait\" href=\"gear_common/storage/trait.IterableByKeyMap.html\" title=\"trait gear_common::storage::IterableByKeyMap\">IterableByKeyMap</a>&lt;&lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Value\" title=\"type gear_common::storage::DoubleMapStorage::Value\">Value</a>&gt; for <a class=\"struct\" href=\"gear_common/storage/struct.WaitlistImpl.html\" title=\"struct gear_common::storage::WaitlistImpl\">WaitlistImpl</a>&lt;T, Value, BlockNumber, Error, OutputError, Callbacks, KeyGen&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&lt;Value = (Value, <a class=\"struct\" href=\"gear_common/storage/struct.Interval.html\" title=\"struct gear_common::storage::Interval\">Interval</a>&lt;BlockNumber&gt;)&gt; + <a class=\"trait\" href=\"gear_common/storage/trait.IterableByKeyMap.html\" title=\"trait gear_common::storage::IterableByKeyMap\">IterableByKeyMap</a>&lt;T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Value\" title=\"type gear_common::storage::DoubleMapStorage::Value\">Value</a>, Key = T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key1\" title=\"type gear_common::storage::DoubleMapStorage::Key1\">Key1</a>&gt;,\n    Error: <a class=\"trait\" href=\"gear_common/storage/trait.WaitlistError.html\" title=\"trait gear_common::storage::WaitlistError\">WaitlistError</a>,\n    OutputError: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt;,\n    Callbacks: <a class=\"trait\" href=\"gear_common/storage/trait.WaitlistCallbacks.html\" title=\"trait gear_common::storage::WaitlistCallbacks\">WaitlistCallbacks</a>&lt;Value = Value, BlockNumber = BlockNumber&gt;,\n    KeyGen: <a class=\"trait\" href=\"gear_common/storage/trait.KeyFor.html\" title=\"trait gear_common::storage::KeyFor\">KeyFor</a>&lt;Key = (T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key1\" title=\"type gear_common::storage::DoubleMapStorage::Key1\">Key1</a>, T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key2\" title=\"type gear_common::storage::DoubleMapStorage::Key2\">Key2</a>), Value = Value&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Key\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Key\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.IterableByKeyMap.html#associatedtype.Key\" class=\"associatedtype\">Key</a> = &lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key1\" title=\"type gear_common::storage::DoubleMapStorage::Key1\">Key1</a></h4></section></summary><div class='docblock'>Map’s first key type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.DrainIter\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.DrainIter\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.IterableByKeyMap.html#associatedtype.DrainIter\" class=\"associatedtype\">DrainIter</a> = &lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.IterableByKeyMap.html\" title=\"trait gear_common::storage::IterableByKeyMap\">IterableByKeyMap</a>&lt;&lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Value\" title=\"type gear_common::storage::DoubleMapStorage::Value\">Value</a>&gt;&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.IterableByKeyMap.html#associatedtype.DrainIter\" title=\"type gear_common::storage::IterableByKeyMap::DrainIter\">DrainIter</a></h4></section></summary><div class='docblock'>Removal iterator type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Iter\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Iter\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.IterableByKeyMap.html#associatedtype.Iter\" class=\"associatedtype\">Iter</a> = &lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.IterableByKeyMap.html\" title=\"trait gear_common::storage::IterableByKeyMap\">IterableByKeyMap</a>&lt;&lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Value\" title=\"type gear_common::storage::DoubleMapStorage::Value\">Value</a>&gt;&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.IterableByKeyMap.html#associatedtype.Iter\" title=\"type gear_common::storage::IterableByKeyMap::Iter\">Iter</a></h4></section></summary><div class='docblock'>Getting iterator type.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.drain_key\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#211-213\">source</a><a href=\"#method.drain_key\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"gear_common/storage/trait.IterableByKeyMap.html#tymethod.drain_key\" class=\"fn\">drain_key</a>(key: Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.IterableByKeyMap.html#associatedtype.Key\" title=\"type gear_common::storage::IterableByKeyMap::Key\">Key</a>) -&gt; Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.IterableByKeyMap.html#associatedtype.DrainIter\" title=\"type gear_common::storage::IterableByKeyMap::DrainIter\">DrainIter</a></h4></section></summary><div class='docblock'>Creates the removal iterator over double map Items.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.iter_key\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#215-217\">source</a><a href=\"#method.iter_key\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"gear_common/storage/trait.IterableByKeyMap.html#tymethod.iter_key\" class=\"fn\">iter_key</a>(key: Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.IterableByKeyMap.html#associatedtype.Key\" title=\"type gear_common::storage::IterableByKeyMap::Key\">Key</a>) -&gt; Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.IterableByKeyMap.html#associatedtype.Iter\" title=\"type gear_common::storage::IterableByKeyMap::Iter\">Iter</a></h4></section></summary><div class='docblock'>Creates the getting iterator over double map Items.</div></details></div></details>","IterableByKeyMap<<T as DoubleMapStorage>::Value>","gear_common::auxiliary::waitlist::AuxiliaryWaitlist"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IterableMap%3C%3CT+as+DoubleMapStorage%3E::Value%3E-for-WaitlistImpl%3CT,+Value,+BlockNumber,+Error,+OutputError,+Callbacks,+KeyGen%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#222-241\">source</a><a href=\"#impl-IterableMap%3C%3CT+as+DoubleMapStorage%3E::Value%3E-for-WaitlistImpl%3CT,+Value,+BlockNumber,+Error,+OutputError,+Callbacks,+KeyGen%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, Value, BlockNumber, Error, OutputError, Callbacks, KeyGen&gt; <a class=\"trait\" href=\"gear_common/storage/trait.IterableMap.html\" title=\"trait gear_common::storage::IterableMap\">IterableMap</a>&lt;&lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Value\" title=\"type gear_common::storage::DoubleMapStorage::Value\">Value</a>&gt; for <a class=\"struct\" href=\"gear_common/storage/struct.WaitlistImpl.html\" title=\"struct gear_common::storage::WaitlistImpl\">WaitlistImpl</a>&lt;T, Value, BlockNumber, Error, OutputError, Callbacks, KeyGen&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&lt;Value = (Value, <a class=\"struct\" href=\"gear_common/storage/struct.Interval.html\" title=\"struct gear_common::storage::Interval\">Interval</a>&lt;BlockNumber&gt;)&gt; + <a class=\"trait\" href=\"gear_common/storage/trait.IterableMap.html\" title=\"trait gear_common::storage::IterableMap\">IterableMap</a>&lt;T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Value\" title=\"type gear_common::storage::DoubleMapStorage::Value\">Value</a>&gt;,\n    Error: <a class=\"trait\" href=\"gear_common/storage/trait.WaitlistError.html\" title=\"trait gear_common::storage::WaitlistError\">WaitlistError</a>,\n    OutputError: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt;,\n    Callbacks: <a class=\"trait\" href=\"gear_common/storage/trait.WaitlistCallbacks.html\" title=\"trait gear_common::storage::WaitlistCallbacks\">WaitlistCallbacks</a>&lt;Value = Value, BlockNumber = BlockNumber&gt;,\n    KeyGen: <a class=\"trait\" href=\"gear_common/storage/trait.KeyFor.html\" title=\"trait gear_common::storage::KeyFor\">KeyFor</a>&lt;Key = (T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key1\" title=\"type gear_common::storage::DoubleMapStorage::Key1\">Key1</a>, T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key2\" title=\"type gear_common::storage::DoubleMapStorage::Key2\">Key2</a>), Value = Value&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.DrainIter\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.DrainIter\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.IterableMap.html#associatedtype.DrainIter\" class=\"associatedtype\">DrainIter</a> = &lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.IterableMap.html\" title=\"trait gear_common::storage::IterableMap\">IterableMap</a>&lt;&lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Value\" title=\"type gear_common::storage::DoubleMapStorage::Value\">Value</a>&gt;&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.IterableMap.html#associatedtype.DrainIter\" title=\"type gear_common::storage::IterableMap::DrainIter\">DrainIter</a></h4></section></summary><div class='docblock'>Removal iterator type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Iter\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Iter\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.IterableMap.html#associatedtype.Iter\" class=\"associatedtype\">Iter</a> = &lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.IterableMap.html\" title=\"trait gear_common::storage::IterableMap\">IterableMap</a>&lt;&lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Value\" title=\"type gear_common::storage::DoubleMapStorage::Value\">Value</a>&gt;&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.IterableMap.html#associatedtype.Iter\" title=\"type gear_common::storage::IterableMap::Iter\">Iter</a></h4></section></summary><div class='docblock'>Getting iterator type.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.drain\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#234-236\">source</a><a href=\"#method.drain\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"gear_common/storage/trait.IterableMap.html#tymethod.drain\" class=\"fn\">drain</a>() -&gt; Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.IterableMap.html#associatedtype.DrainIter\" title=\"type gear_common::storage::IterableMap::DrainIter\">DrainIter</a></h4></section></summary><div class='docblock'>Creates the removal iterator over map Items.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.iter\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#238-240\">source</a><a href=\"#method.iter\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"gear_common/storage/trait.IterableMap.html#tymethod.iter\" class=\"fn\">iter</a>() -&gt; Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.IterableMap.html#associatedtype.Iter\" title=\"type gear_common::storage::IterableMap::Iter\">Iter</a></h4></section></summary><div class='docblock'>Creates the getting iterator over map Items.</div></details></div></details>","IterableMap<<T as DoubleMapStorage>::Value>","gear_common::auxiliary::waitlist::AuxiliaryWaitlist"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Waitlist-for-WaitlistImpl%3CT,+Value,+BlockNumber,+Error,+OutputError,+Callbacks,+KeyGen%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#115-174\">source</a><a href=\"#impl-Waitlist-for-WaitlistImpl%3CT,+Value,+BlockNumber,+Error,+OutputError,+Callbacks,+KeyGen%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, Value, BlockNumber, Error, OutputError, Callbacks, KeyGen&gt; <a class=\"trait\" href=\"gear_common/storage/trait.Waitlist.html\" title=\"trait gear_common::storage::Waitlist\">Waitlist</a> for <a class=\"struct\" href=\"gear_common/storage/struct.WaitlistImpl.html\" title=\"struct gear_common::storage::WaitlistImpl\">WaitlistImpl</a>&lt;T, Value, BlockNumber, Error, OutputError, Callbacks, KeyGen&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&lt;Value = (Value, <a class=\"struct\" href=\"gear_common/storage/struct.Interval.html\" title=\"struct gear_common::storage::Interval\">Interval</a>&lt;BlockNumber&gt;)&gt;,\n    Error: <a class=\"trait\" href=\"gear_common/storage/trait.WaitlistError.html\" title=\"trait gear_common::storage::WaitlistError\">WaitlistError</a>,\n    OutputError: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt;,\n    Callbacks: <a class=\"trait\" href=\"gear_common/storage/trait.WaitlistCallbacks.html\" title=\"trait gear_common::storage::WaitlistCallbacks\">WaitlistCallbacks</a>&lt;Value = Value, BlockNumber = BlockNumber&gt;,\n    KeyGen: <a class=\"trait\" href=\"gear_common/storage/trait.KeyFor.html\" title=\"trait gear_common::storage::KeyFor\">KeyFor</a>&lt;Key = (T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key1\" title=\"type gear_common::storage::DoubleMapStorage::Key1\">Key1</a>, T::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key2\" title=\"type gear_common::storage::DoubleMapStorage::Key2\">Key2</a>), Value = Value&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Key1\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Key1\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.Waitlist.html#associatedtype.Key1\" class=\"associatedtype\">Key1</a> = &lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key1\" title=\"type gear_common::storage::DoubleMapStorage::Key1\">Key1</a></h4></section></summary><div class='docblock'>First key type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Key2\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Key2\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.Waitlist.html#associatedtype.Key2\" class=\"associatedtype\">Key2</a> = &lt;T as <a class=\"trait\" href=\"gear_common/storage/trait.DoubleMapStorage.html\" title=\"trait gear_common::storage::DoubleMapStorage\">DoubleMapStorage</a>&gt;::<a class=\"associatedtype\" href=\"gear_common/storage/trait.DoubleMapStorage.html#associatedtype.Key2\" title=\"type gear_common::storage::DoubleMapStorage::Key2\">Key2</a></h4></section></summary><div class='docblock'>Second key type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Value\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Value\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.Waitlist.html#associatedtype.Value\" class=\"associatedtype\">Value</a> = Value</h4></section></summary><div class='docblock'>Stored values type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.BlockNumber\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.BlockNumber\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.Waitlist.html#associatedtype.BlockNumber\" class=\"associatedtype\">BlockNumber</a> = BlockNumber</h4></section></summary><div class='docblock'>Block number type. <a href=\"gear_common/storage/trait.Waitlist.html#associatedtype.BlockNumber\">Read more</a></div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.Waitlist.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = Error</h4></section></summary><div class='docblock'>Inner error type of waitlist storing algorithm.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.OutputError\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.OutputError\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"gear_common/storage/trait.Waitlist.html#associatedtype.OutputError\" class=\"associatedtype\">OutputError</a> = OutputError</h4></section></summary><div class='docblock'>Output error type of the waitlist.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.contains\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#131-133\">source</a><a href=\"#method.contains\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"gear_common/storage/trait.Waitlist.html#tymethod.contains\" class=\"fn\">contains</a>(program_id: &amp;Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.Waitlist.html#associatedtype.Key1\" title=\"type gear_common::storage::Waitlist::Key1\">Key1</a>, message_id: &amp;Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.Waitlist.html#associatedtype.Key2\" title=\"type gear_common::storage::Waitlist::Key2\">Key2</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Returns bool, defining does first key’s waitlist contain second key.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.insert\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#135-157\">source</a><a href=\"#method.insert\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"gear_common/storage/trait.Waitlist.html#tymethod.insert\" class=\"fn\">insert</a>(\n    message: Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.Waitlist.html#associatedtype.Value\" title=\"type gear_common::storage::Waitlist::Value\">Value</a>,\n    scheduled_at: Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.Waitlist.html#associatedtype.BlockNumber\" title=\"type gear_common::storage::Waitlist::BlockNumber\">BlockNumber</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.Waitlist.html#associatedtype.OutputError\" title=\"type gear_common::storage::Waitlist::OutputError\">OutputError</a>&gt;</h4></section></summary><div class='docblock'>Inserts given value in waitlist.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.remove\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#159-169\">source</a><a href=\"#method.remove\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"gear_common/storage/trait.Waitlist.html#tymethod.remove\" class=\"fn\">remove</a>(\n    program_id: Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.Waitlist.html#associatedtype.Key1\" title=\"type gear_common::storage::Waitlist::Key1\">Key1</a>,\n    message_id: Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.Waitlist.html#associatedtype.Key2\" title=\"type gear_common::storage::Waitlist::Key2\">Key2</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;(Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.Waitlist.html#associatedtype.Value\" title=\"type gear_common::storage::Waitlist::Value\">Value</a>, <a class=\"struct\" href=\"gear_common/storage/struct.Interval.html\" title=\"struct gear_common::storage::Interval\">Interval</a>&lt;Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.Waitlist.html#associatedtype.BlockNumber\" title=\"type gear_common::storage::Waitlist::BlockNumber\">BlockNumber</a>&gt;), Self::<a class=\"associatedtype\" href=\"gear_common/storage/trait.Waitlist.html#associatedtype.OutputError\" title=\"type gear_common::storage::Waitlist::OutputError\">OutputError</a>&gt;</h4></section></summary><div class='docblock'>Removes and returns value from waitlist by given keys,\nif present, else returns error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clear\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gear_common/storage/complex/waitlist.rs.html#171-173\">source</a><a href=\"#method.clear\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"gear_common/storage/trait.Waitlist.html#tymethod.clear\" class=\"fn\">clear</a>()</h4></section></summary><div class='docblock'>Removes all values from all key’s waitlisted.</div></details></div></details>","Waitlist","gear_common::auxiliary::waitlist::AuxiliaryWaitlist"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()