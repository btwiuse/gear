(function() {var type_impls = {
"pallet_gear_gas":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-Clone-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; GasNodeId&lt;T, U&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#175\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-Debug-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decode-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-Decode-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; Decode for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: Decode,\n    U: Decode,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode\" class=\"method trait-impl\"><a href=\"#method.decode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">decode</a>&lt;__CodecInputEdqy&gt;(\n    __codec_input_edqy: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __CodecInputEdqy</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;GasNodeId&lt;T, U&gt;, Error&gt;<div class=\"where\">where\n    __CodecInputEdqy: Input,</div></h4></section></summary><div class='docblock'>Attempt to deserialise the value from input.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode_into\" class=\"method trait-impl\"><a href=\"#method.decode_into\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">decode_into</a>&lt;I&gt;(\n    input: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut I</a>,\n    dst: &amp;mut <a class=\"union\" href=\"https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html\" title=\"union core::mem::maybe_uninit::MaybeUninit\">MaybeUninit</a>&lt;Self&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;DecodeFinished, Error&gt;<div class=\"where\">where\n    I: Input,</div></h4></section></summary><div class='docblock'>Attempt to deserialize the value from input into a pre-allocated piece of memory. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.skip\" class=\"method trait-impl\"><a href=\"#method.skip\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">skip</a>&lt;I&gt;(input: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut I</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, Error&gt;<div class=\"where\">where\n    I: Input,</div></h4></section></summary><div class='docblock'>Attempt to skip the encoded value from input. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.encoded_fixed_size\" class=\"method trait-impl\"><a href=\"#method.encoded_fixed_size\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">encoded_fixed_size</a>() -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class='docblock'>Returns the fixed encoded size of the type. <a>Read more</a></div></details></div></details>","Decode","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-Display-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encode-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-Encode-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; Encode for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: Encode,\n    U: Encode,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.size_hint\" class=\"method trait-impl\"><a href=\"#method.size_hint\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">size_hint</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>If possible give a hint of expected size of the encoding. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_to\" class=\"method trait-impl\"><a href=\"#method.encode_to\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">encode_to</a>&lt;__CodecOutputEdqy&gt;(\n    &amp;self,\n    __codec_dest_edqy: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __CodecOutputEdqy</a>,\n)<div class=\"where\">where\n    __CodecOutputEdqy: Output + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Convert self to a slice and append it to the destination.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode\" class=\"method trait-impl\"><a href=\"#method.encode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">encode</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"Vec&lt;u8&gt;\">ⓘ</a></h4></section></summary><div class='docblock'>Convert self to an owned vector.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.using_encoded\" class=\"method trait-impl\"><a href=\"#method.using_encoded\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">using_encoded</a>&lt;R, F&gt;(&amp;self, f: F) -&gt; R<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(&amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]) -&gt; R,</div></h4></section></summary><div class='docblock'>Convert self to a slice and then invoke the given closure with it.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.encoded_size\" class=\"method trait-impl\"><a href=\"#method.encoded_size\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">encoded_size</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Calculates the encoded size. <a>Read more</a></div></details></div></details>","Encode","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CMessageId%3E-for-GasNodeId%3CMessageId,+U%3E\" class=\"impl\"><a href=\"#impl-From%3CMessageId%3E-for-GasNodeId%3CMessageId,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;MessageId&gt; for GasNodeId&lt;MessageId, U&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(id: MessageId) -&gt; GasNodeId&lt;MessageId, U&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<MessageId>","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CReservationId%3E-for-GasNodeId%3CT,+ReservationId%3E\" class=\"impl\"><a href=\"#impl-From%3CReservationId%3E-for-GasNodeId%3CT,+ReservationId%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;ReservationId&gt; for GasNodeId&lt;T, ReservationId&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(id: ReservationId) -&gt; GasNodeId&lt;T, ReservationId&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<ReservationId>","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; GasNodeId&lt;T, U&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.to_node_id\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">to_node_id</a>(self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;</h4></section><section id=\"method.to_reservation_id\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">to_reservation_id</a>(self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;U&gt;</h4></section></div></details>",0,"pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hash-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-Hash-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash\" class=\"fn\">hash</a>&lt;__H&gt;(&amp;self, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __H</a>)<div class=\"where\">where\n    __H: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,</div></h4></section></summary><div class='docblock'>Feeds this value into the given <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_slice\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.3.0\">1.3.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237\">source</a></span><a href=\"#method.hash_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice\" class=\"fn\">hash_slice</a>&lt;H&gt;(data: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[Self]</a>, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Feeds a slice of this type into the given <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice\">Read more</a></div></details></div></details>","Hash","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Ord-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-Ord-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.cmp\" class=\"method trait-impl\"><a href=\"#method.cmp\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp\" class=\"fn\">cmp</a>(&amp;self, other: &amp;GasNodeId&lt;T, U&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html\" title=\"enum core::cmp::Ordering\">Ordering</a></h4></section></summary><div class='docblock'>This method returns an <a href=\"https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html\" title=\"enum core::cmp::Ordering\"><code>Ordering</code></a> between <code>self</code> and <code>other</code>. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.max\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.21.0\">1.21.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#854-856\">source</a></span><a href=\"#method.max\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max\" class=\"fn\">max</a>(self, other: Self) -&gt; Self<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Compares and returns the maximum of two values. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.min\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.21.0\">1.21.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#875-877\">source</a></span><a href=\"#method.min\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min\" class=\"fn\">min</a>(self, other: Self) -&gt; Self<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Compares and returns the minimum of two values. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clamp\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.50.0\">1.50.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#901-904\">source</a></span><a href=\"#method.clamp\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp\" class=\"fn\">clamp</a>(self, min: Self, max: Self) -&gt; Self<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>,</div></h4></section></summary><div class='docblock'>Restrict a value to a certain interval. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp\">Read more</a></div></details></div></details>","Ord","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-PartialEq-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;GasNodeId&lt;T, U&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#261\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialOrd-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-PartialOrd-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.partial_cmp\" class=\"method trait-impl\"><a href=\"#method.partial_cmp\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp\" class=\"fn\">partial_cmp</a>(&amp;self, other: &amp;GasNodeId&lt;T, U&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html\" title=\"enum core::cmp::Ordering\">Ordering</a>&gt;</h4></section></summary><div class='docblock'>This method returns an ordering between <code>self</code> and <code>other</code> values if one exists. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.lt\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1178\">source</a></span><a href=\"#method.lt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt\" class=\"fn\">lt</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests less than (for <code>self</code> and <code>other</code>) and is used by the <code>&lt;</code> operator. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.le\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1196\">source</a></span><a href=\"#method.le\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le\" class=\"fn\">le</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests less than or equal to (for <code>self</code> and <code>other</code>) and is used by the\n<code>&lt;=</code> operator. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gt\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1214\">source</a></span><a href=\"#method.gt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt\" class=\"fn\">gt</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests greater than (for <code>self</code> and <code>other</code>) and is used by the <code>&gt;</code>\noperator. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ge\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1232\">source</a></span><a href=\"#method.ge\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge\" class=\"fn\">ge</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests greater than or equal to (for <code>self</code> and <code>other</code>) and is used by\nthe <code>&gt;=</code> operator. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge\">Read more</a></div></details></div></details>","PartialOrd","pallet_gear_gas::pallet::Key"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TypeInfo-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-TypeInfo-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; TypeInfo for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: TypeInfo + 'static,\n    U: TypeInfo + 'static,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Identity\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Identity\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Identity</a> = GasNodeId&lt;T, U&gt;</h4></section></summary><div class='docblock'>The type identifying for which type info is provided. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_info\" class=\"method trait-impl\"><a href=\"#method.type_info\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">type_info</a>() -&gt; Type</h4></section></summary><div class='docblock'>Returns the static type identifier for <code>Self</code>.</div></details></div></details>","TypeInfo","pallet_gear_gas::pallet::Key"],["<section id=\"impl-Copy-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-Copy-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,</div></h3></section>","Copy","pallet_gear_gas::pallet::Key"],["<section id=\"impl-EncodeLike-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-EncodeLike-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; EncodeLike for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: Encode,\n    U: Encode,</div></h3></section>","EncodeLike","pallet_gear_gas::pallet::Key"],["<section id=\"impl-Eq-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-Eq-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for GasNodeId&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>,</div></h3></section>","Eq","pallet_gear_gas::pallet::Key"],["<section id=\"impl-StructuralPartialEq-for-GasNodeId%3CT,+U%3E\" class=\"impl\"><a href=\"#impl-StructuralPartialEq-for-GasNodeId%3CT,+U%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for GasNodeId&lt;T, U&gt;</h3></section>","StructuralPartialEq","pallet_gear_gas::pallet::Key"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()