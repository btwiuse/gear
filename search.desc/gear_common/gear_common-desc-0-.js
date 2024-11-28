searchState.loadedDescShard("gear_common", 0, "Program in active state.\nActive program in storage.\nType representing a quantity of value.\nThe maximum amount of gas that can be used within a single …\nContains various limits for the block.\nCode identifier.\nTrait that the RuntimeApi should implement in order to …\nTrait that is used to “delegate fee” by optionally …\nThe gas lock is provided by dispatch stash.\nProgram has been exited (gr_exit was called)\nA trait whose purpose is to extract the <code>Call</code> variant of an …\nType alias for gas entity.\nType manages a gas that is available at the moment of call.\nType representing converter between gas and value in …\nPage of gear page size - 16 kiB.\nThe id of the gas lock.\nThe gas lock is provided by the mailbox.\nStruct defines infix of memory pages storage.\nMessage identifier.\nBuffer for gear page data.\nProgram in different states in storage.\nProgram identifier.\nThe gas lock is provided by reservation.\nReservation identifier.\nProgram has been terminated (<code>init</code> was failed)\nThe gas lock is provided by the waitlist.\nContinuous intervals amount in program allocations.\nAuxiliary implementations of the complex data structures, …\nSet of supported dispatch kinds.\nCode hash of the program.\nGear events additional data.\nBlock number when the program will be expired.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates PageBuf from inner buffer. If the buffer has the …\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGas reservation map.\nConverts given gas amount into its value equivalent.\nReturn inner u32 value.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns <code>MessageId</code>as bytes array.\nReturns <code>CodeId</code>as bytes array.\nReturns <code>ReservationId</code>as bytes array.\nReturns whether the program is active.\nReturns whether the program is exited.\nReturns whether the program is active and initialized.\nReturns whether the program is terminated.\nChecks whether <code>MessageId</code> is zero.\nChecks whether <code>CodeId</code> is zero.\nChecks whether <code>ReservationId</code> is zero.\nInfix of memory pages storage (is used for memory wake …\nCreates a new <code>MessageId</code> from a 32-byte array.\nCreates a new <code>CodeId</code> from a 32-byte array.\nCreates a new <code>ReservationId</code> from a 32-byte array.\nConstructing function from u32 number.\nReturns new page buffer with zeroed data.\nModule contains macros that help to implement Config type …\nModule for scheduler implementation.\nInitialization state of the program.\nAmount of static pages.\nGear’s storage API module.\nSame as <code>wrap_storage_map!</code>, but with length type parameter …\nSame as <code>wrap_storage_double_map!</code>, but with extra …\nCreates new type with specified name and key1-key2-value …\nCreates new type with specified name and key-value types …\nCreates new type with specified name and …\nCreates new type with specified name and value type and …\nCreates a new zero <code>MessageId</code>.\nCreates a new zero <code>CodeId</code>.\nCreates a new zero <code>ReservationId</code>.\nAn auxiliary storage wrapper type.\nAn “auxiliary” block number type.\nDouble key <code>BTreeMap</code>.\nClears the map, removing all elements.\nReturns <code>true</code> if the map contains a value for the specified …\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nAuxiliary implementation of the gas provider.\nReturns a reference to the value corresponding to the keys.\nInserts a value under provided keys in the map.\nCalls <code>U::from(self)</code>.\nAuxiliary implementation of the mailbox.\nInstantiate new empty double key map.\nRemoves keys from the map, returning the value at the keys …\nAuxiliary implementation of the task pool.\nAuxiliary implementation of the waitlist.\nGas provider implementor used in native, non-wasm runtimes.\nGlobal <code>GasNodes</code> storage manager.\nError type serving error variants returned from gas tree …\nGlobal <code>TotalIssuance</code> storage manager.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMailbox implementation that can be used in a native, …\nAn implementor of the error returned from calling <code>Mailbox</code> …\n<code>Mailbox</code> double storage map manager.\nType represents message stored in the mailbox.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTask pool implementation that can be used in a native, …\nOccurs when given task already exists in task pool.\nOccurs when task wasn’t found in storage.\nAn implementor of the error returned from calling <code>TaskPool</code> …\n<code>TaskPool</code> double storage map manager\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWaitlist implementation that can be used in a native, …\nAn implementor of the error returned from calling <code>Waitlist</code> …\n<code>Waitlist</code> double storage map manager.\nType represents message stored in the waitlist.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTrait to work with program binary codes in a storage.\nCode already exists in storage.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nReturns true if the code associated with given id was …\nAttempt to remove all items from all the associated maps.\nUpdate the corresponding code in the storage.\nCode become active and ready for use.\nActive status achieved.\nType of changes applied to code in storage.\nStatus of dispatch dequeue and execution.\nOccurs when expiration block number of a program changed.\nDispatch was dequeued and failed its execution.\nHandle entry point.\nCode become inactive and can no longer be used.\nProgram become inactive forever due to <code>gr_exit</code> call.\nInit entry point.\nMessage was claimed by user.\nPrograms entry for messages.\nMessage was replied by user.\nComposite reason for messages waiting.\nRuntime reason for messages waiting.\nSystem reason for messages waiting.\nComposite reason for messages waking.\nRuntime reason for messages waking.\nSystem reason for messages waking.\nDispatch was dequeued and wasn’t executed. Occurs if …\nMessage can no longer pay rent for holding in storage (see …\nMessage can no longer pay rent for holding in storage (see …\nPaused status.\nType of changes applied to program in storage.\nProgram had finished initialization.\nOccurs when new program set in the storage.\nComposite reason type for any action happened on chain.\nCode was reinstrumented.\nHandle reply entry point.\nRuntime reason variant.\nRuntime reason variant.\nRuntime reason variant.\nRuntime reason variant.\nBehavior of types, which represent runtime reasons for …\nSystem signal entry point.\nDispatch was dequeued and succeed with execution.\nSystem reason variant.\nSystem reason variant.\nSystem reason variant.\nSystem reason variant.\nBehavior of types, which represent system reasons for some …\nProgram become inactive forever due to init failure.\nSpecified by program timeout for waking has come (see …\nComposite reason for messages reading from <code>Mailbox</code>.\nRuntime reason for messages reading from <code>Mailbox</code>.\nSystem reason for messages reading from <code>Mailbox</code>.\nProgram called <code>gr_wait</code> while executing message.\nProgram called <code>gr_wait_for</code> while executing message.\nProgram called <code>gr_wait_up_to</code> with insufficient gas for full\nProgram called <code>gr_wait_up_to</code> with enough gas for full …\nProgram called <code>gr_wake</code> with corresponding message id.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverter into composite reason type: not only runtime, …\nConverter into composite reason type: not only system, but …\nType representing a quantity of value.\nType representing a quantity of value.\nChildren references convenience struct\nSimplified type for result of <code>GasTree::consume</code> call.\nA node created by “cutting” value from some other tree …\nThe gas lock is provided by dispatch stash.\nContains the error value\nErrors stating that gas tree has been invalidated.\nError type\nError type.\nA root node for each gas tree.\nType representing the external owner of a value (gas) item.\nType representing the external owner of a value (gas) item.\nType representing a quantity of token balance.\nType representing a quantity of token balance.\nNode of the [‘Tree’] gas tree\nID of the <code>GasNode</code>.\nA ledger to account for gas creation and consumption.\nRepresents either added or removed value to/from total …\nRepresents errors returned by via the Imbalance trait. …\nTypes to denote a result of some unbalancing operation - …\nThe id of the gas lock.\nThe gas lock is provided by the mailbox.\nOpaque, move-only struct with private field to denote that …\n<code>NegativeImbalance</code> indicates that some value has been …\nType that identifies a node of the tree.\nType that identifies a tree node.\nContains the success value\nSimplified type for <code>GasTree::get_origin_node</code> call.\nOpaque, move-only struct with private field to denote that …\nTypes to denote a result of some unbalancing operation - …\nRepresents logic of centralized GasTree-algorithm.\nThe gas lock is provided by reservation.\nA node used for gas reservation feature.\nA node, which is a part of the tree structure, that can be …\nAbstraction for a chain of value items each piece of which …\nPretty same as <code>SpecifiedLocal</code>, but doesn’t have internal …\nThe gas lock is provided by the waitlist.\nApplies imbalance to some amount.\nRemoves all values.\nConsume underlying value.\nMarks a node with <code>key</code> as consumed, if possible, and tries …\n<code>GasTree::consume</code> called on node, which has some balance …\n<code>GasTree::consume</code> called on node, which has some system …\nIncrease the total issuance of the underlying value by …\nCreates deposit external node to be used as pre-defined …\nCut underlying value to a reserved node.\nDecreases node’s spec refs, if it can have any\nDecreases node’s unspec refs, if it can have any\nReturn bool, defining does node exist.\nReturns bool, defining does node exist and is external …\nReturns external origin and funds multiplier of the node …\nForbidden operation for the value node.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nThe external origin for a key.\nThe funds multiplier for a key.\nGet value associated with given id.\nGet value associated with given id within consumed node.\nGet value associated with given id and the key of an …\nGet value associated with given id and the key of an …\nGet locked value associated with given id.\nThe id of external node for a key.\nThe id of node, external origin and funds multiplier for a …\nGet system reserve value associated with given id.\nIncreases node’s spec refs, if it can have any\nIncreases node’s unspec refs, if it can have any\nAccount doesn’t have enough funds to complete operation.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns whether the node is marked consumed or not\nReturns whether the node is patron or not.\nLocking some value from underlying node balance.\nReturns node’s locked gas balance, if it can have any.\nGet’s a mutable access to node’s locked gas balance, …\nMarks the node as consumed, if it has the flag\nCreate a new negative imbalance from value amount.\nCreates a new <code>GasNode::External</code> root node for a new tree.\nCreate a new positive imbalance from value amount.\nGas (gas tree) has already been created for the provided …\nValue node doesn’t exist for a key.\nProcedure can’t be called on consumed node.\nReturns node’s parent, if it can have any.\nParent node must have children, but they weren’t found.\nParent must be in the tree, but not found.\nReturns imbalance raw value.\nReturns node’s total refs\nReserve some value from underlying balance.\nResets all related to gas provider storages.\nReturns id of the root node.\nReturns node’s spec refs\nBurn underlying value.\nSpends <code>amount</code> of gas from the ancestor of node with <code>key</code> id.\nSplit underlying value.\nSplit underlying value.\nReserve some value from underlying balance.\nReturns node’s system reserved gas balance, if it can …\nGets a mutable access to node’s system reserved gas …\nUnreserve some value from underlying balance.\nThe total amount of value currently in circulation.\nReturns total gas value inside GasNode.\n<code>GasTree::create</code> called with some value amount leading to …\nEither <code>GasTree::consume</code> or <code>GasTree::spent</code> called on a node …\nOutput of <code>Tree::consume</code> procedure that wasn’t expected.\nNode type that can’t occur if algorithm work well\nUnlocking some value from node’s locked balance.\nUnlocking all value from node’s locked balance. Returns …\nUnlocking all value from node’s locked balance. Returns …\nReturns node’s unspec refs\nReturns node’s inner gas balance, if it can have any\nValue must have been caught or moved upstream, but was …\nValue must have been blocked, but was either moved or …\nValue must have been caught, but was missed or blocked …\nGet’s a mutable access to node’s inner gas balance, if …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nReturns the value of this parameter type.\nReturns the value of this parameter type.\nReturns the value of this parameter type.\nReturns the value of this parameter type.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTrait for ProgramStorage errors.\nTrait to work with program data in a storage.\nStore a program to be associated with the given key …\nThere is no data for specified <code>program_id</code> and <code>page</code>.\nRemove all memory page buffers under the given keys …\nProgram already exists in storage.\nLoad the program associated with the given key <code>program_id</code> …\nReturn data buffer for each memory page, which has data.\nProgram is not an instance of ActiveProgram.\nFinal full prefix that prefixes all keys of memory pages.\nFailed to find the program binary code.\nDoes the program (explicitly) exist in storage?\nProgram is not found in the storage.\nRemove a memory page buffer under the given keys <code>program_id</code>…\nAttempt to remove all items from all the associated maps.\nStore a memory page buffer to be associated with the given …\nUpdate the active program under the given key <code>program_id</code>.\nUpdate the program under the given key <code>program_id</code> only if …\nBlock number type of the messenger.\nBlock number type.\nBlock number type.\nCost type.\nCost type.\nStoring costs per block.\nInner error type generated by gear’s storage types.\nInner error type of queue storing algorithm.\nThe first block of incomplete tasks, which have already …\nCallback on success <code>add</code>.\nCallback on success <code>delete</code>.\nOutput error of each storage algorithm.\nOutput error type of the queue.\nRepresents scheduler’s logic of centralized delayed …\nStoring costs getter trait.\nThe type whose variants correspond to various storages …\nTask type.\nTask type.\nRepresents tasks managing logic.\nGear task pool.\nRepresents store of task pool’s action callbacks.\nRepresents task pool error type.\n<code>TaskPool</code> implementation based on <code>DoubleMapStorage</code>.\nInserts given task in task pool.\nDerives the cost per block based on the lock identifier\nRemoves all tasks from task pool.\nCost for storing code per block.\nReturns bool, defining does task exist in task pool.\nRemoves task from task pool by given keys, if present, …\nCost for storing message in dispatch stash. Everything …\nOccurs when given task already exists in task pool.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCost for storing message in mailbox per block.\nCost for storing program per block.\nCost for reservation holding.\nExtra reserve for being able to pay for blocks with …\nResets all related to messenger storages.\nOccurs when task wasn’t found in storage.\nCost for storing message in waitlist per block.\nBlock number type.\nCallback relative type.\nBlock number type of the messenger.\nBlock number type.\nCallback relative type.\nRepresents callback action for cases <code>(&amp;T) -&gt; R</code>, where <code>R</code> is …\nCapacity of the messenger.\nRepresents default counting logic, by providing ability to …\nRepresents default counting logic, by providing ability to …\nRepresents logic of managing step-by-step changeable value.\n<code>Counter</code> implementation based on <code>ValueStorage</code>.\nStored values type for <code>Self::DispatchStash</code>.\nRepresents dequeue implementation.\nRepresents store of dequeue’s action callbacks.\nDrain iterator over dequeue’s values.\nRepresents dequeue error type.\n<code>Dequeue</code> implementation based on <code>MapStorage</code> and <code>ValueStorage</code>…\nCommon iterator over dequeue’s values.\nAmount of messages dequeued with the current block.\nKey for value types for <code>Self::DispatchStash</code>.\nRepresents logic of managing globally stored double-key …\nRemoval iterator type.\nRemoval iterator type.\nRemoval iterator type.\nRepresents callback action for cases without input and …\nInner error type of mailbox storing algorithm.\nInner error type generated by gear’s storage types.\nInner error type of queue storing algorithm.\nInner error type of waitlist storing algorithm.\nDequeue error type.\nReturning error in callback’s <code>Err</code> case.\nRepresents callback action for cases <code>(&amp;T) -&gt; Result&lt;R, E&gt;</code>, …\nCallback used for getting current block number.\nCallback used for getting current block number.\nRepresents callback action for cases without input for …\nTranspose callback for getting first element of tuple.\nTranspose callback for getting second element of tuple.\nTranspose callback for getting third element of tuple.\nType for interval values: e.g. in time <code>(since, till)</code>.\nGetting iterator type.\nGetting iterator type.\nGetting iterator type.\nRepresents iterable logic for double key maps (Key1 -&gt; …\nRepresents iterable logic for single key maps (Key -&gt; …\nRepresents wrapper for any iterator with ability to …\nDequeue’s elements stored key.\nKey type of counting target.\nMap’s first key type.\nGenerated key type.\nMap’s key type.\nFirst key type.\nFirst key type.\nMap’s first key type.\nMap’s first key type.\nMap’s first key type.\nSecond key type.\nSecond key type.\nMap’s second key type.\nMap’s second key type.\nMap’s second key type.\nMap’s third key type.\nRepresents logic of providing key as specified …\nRepresents iterable over second keys logic for double key …\nReturning length type.\nReturning length type.\nRepresents logic of limiting value decreasing.\n<code>Limiter</code> implementation based on <code>ValueStorage</code>.\nRepresents node of the dequeue.\nRepresents mailbox managing logic.\nGear mailbox.\nRepresents store of mailbox’s action callbacks.\nRepresents mailbox error type.\nFirst key of the mailbox storage.\n<code>Mailbox</code> implementation based on <code>DoubleMapStorage</code>.\nKey generator for <code>gear</code>’s mailbox implementation.\nSecond key of the mailbox storage.\nStored values type for <code>Self::Mailbox</code>.\nRepresents logic of managing globally stored single-key …\nRepresents messenger’s logic of centralized message …\nCallback on success <code>clear</code>.\nCallback on success <code>insert</code>.\nCallback on success <code>insert</code>.\nCallback on success <code>pop_back</code>.\nCallback on success <code>pop_front</code>.\nCallback on success <code>push_back</code>.\nCallback on success <code>push_front</code>.\nCallback on success <code>remove</code>.\nCallback on success <code>remove</code>.\nOutput error type of the mailbox.\nOutput error of each storage algorithm.\nOutput error type of the queue.\nOutput error type of the waitlist.\nRepresents message queue managing logic.\nGear message queue.\n<code>Mailbox</code> implementation based on <code>Dequeue</code>.\nKey generator for <code>gear</code>’s message queue implementation.\nAllowance of queue processing.\nStored values type for <code>Self::Queue</code>.\nAmount of messages sent from outside (from users) within …\nRepresents logic of providing access for some actions.\n<code>Toggler</code> implementation based on <code>ValueStorage</code>.\nRepresents transposing callback for mutating values.\nRepresents logic of managing globally stored triple-key …\nStored values type.\nCallback relative type.\nStored values type.\nStored values type.\nCallback relative type.\nCounter stored type.\nDequeue’s elements stored value.\nCallback relative type.\nLimiter stored type.\nMap’s stored value type.\nValue over which key should be generated type.\nMap’s stored value type.\nMap’s stored value type.\nStored value type.\nRepresents logic of managing globally stored value for …\nRepresents waitlist managing logic.\nGear waitlist.\nRepresents store of waitlist’s action callbacks.\nRepresents waitlist error type.\nFirst key of the waitlist storage.\n<code>Waitlist</code> implementation based on <code>DoubleMapStorage</code>.\nKey generator for <code>gear</code>’s waitlist implementation.\nSecond key of the waitlist storage.\nStored values type for <code>Self::Waitlist</code>.\nSets condition to allowed for some action.\nReturns bool, defining does toggle allow some action.\nTriggers the callback’s logic.\nTriggers the callback’s logic.\nTriggers the callback’s logic.\nReturns value by callback’s logic.\nReturns value by callback’s logic.\nRemoves all values from all key’s mailboxes.\nRemoves all values from queue.\nRemoves all values from all key’s waitlisted.\nRemoves all values.\nRemoves all values.\nRemoves all values.\nRemoves all values.\nRemove items from the map matching a <code>first_key</code> prefix.\nRemove items from the map matching a <code>key1</code>/<code>key2</code> prefix.\nReturns bool, defining does first key’s mailbox contain …\nReturns bool, defining does first key’s waitlist contain …\nReturns bool, defining does map contain value under given …\nReturns bool, defining does map contain value under given …\nReturns bool, defining does map contain value under given …\nDecreases stored value.\nDecreases stored value.\nReturns bool, defining does toggle deny some action.\nReturns bool, defining does toggle deny some action.\nSets condition to denied for some action.\nRemoves and returns message from the beginning of the …\nCreates the removal iterator over map Items.\nCreates the removal iterator over double map Items.\nCreates the removal iterator over double map Items.\nOccurs when given value already exists in mailbox.\nOccurs when given value already exists in waitlist.\nOccurs when given key already exists in dequeue.\nOccurs when element wasn’t found in storage.\nOccurs when element wasn’t found in storage.\nOccurs when element wasn’t found in storage.\nReturns bool, defining does value present.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nReturns stored value, if present, or default/starting …\nReturns stored value, if present, or default/nullable …\nGets value stored under given keys, if present.\nGets value stored under given key, if present.\nGets value stored under given keys, if present.\nGets stored value, if present.\nOccurs when head should contain value, but it’s empty …\nOccurs when head should be empty, but it contains value …\nIncreases stored value.\nInserts given value in mailbox.\nInserts given value in waitlist.\nInserts value with given keys.\nInserts value with given key.\nInserts value with given keys.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns bool, defining if Self doesn’t contain elements.\nReturns bool, defining if Self doesn’t contain elements.\nReturns bool, defining if Self doesn’t contain elements.\nReturns bool, defining if Self doesn’t contain elements.\nCreates the getting iterator over map Items.\nCreates the getting iterator over double map Items.\nCreates the getting iterator over double map Items.\nGenerates key for given by reference Value.\nRemoves stored value.\nReturns current Self’s amount of elements as <code>Length</code> type.\nReturns current Self’s amount of elements as <code>Length</code> type.\nMutates value by <code>Option</code> reference, which stored (or not in …\nMutates value by <code>Option</code> reference, which stored (or not in …\nMutates value by <code>Option</code> reference, which stored (or not in …\nMutates stored value by <code>Option</code> reference, which stored (or …\nWorks the same as <code>Self::mutate</code>, but triggers if value …\nWorks the same as <code>Self::mutate</code>, but triggers if value …\nWorks the same as <code>Self::mutate</code>, but triggers if value …\nWorks the same as <code>Self::mutate</code>, but triggers if value …\nWorks the same as <code>Self::mutate</code>, but triggers if value …\nWorks the same as <code>Self::mutate</code>, but triggers if value …\nWorks the same as <code>Self::mutate</code>, but triggers if value …\nWorks the same as <code>Self::mutate</code>, but triggers if value …\nMutates all values in queue with given function.\nMutates all stored value with given function.\nMutates all stored values with given convert function.\nMutates all stored values with given convert function.\nMutates all stored values with given convert function.\nKey of the next node of dequeue, if present.\nPeeks into the mailbox using the given keys to return a …\nRemoves and returns tail value of the dequeue, if present.\nVery expensive operation! Use dequeue based on double …\nRemoves and returns head value of the dequeue, if present.\nInserts value to the end of dequeue with given key.\nInserts value to the beginning of dequeue with given key.\nPuts given value into stored, to start from new one for …\nStores given value.\nInserts given value at the end of the queue.\nRemoves and returns value from mailbox by given keys, if …\nRemoves and returns value from waitlist by given keys, if …\nRemoves value stored under the given keys.\nRemoves value stored under the given key.\nRemoves value stored under the given keys.\nInserts given value at the beginning of the queue.\nResets stored value by setting default/starting value.\nResets all related to messenger storages.\nResets all related to messenger storages.\nStores given value and returns previous one, if present.\nOccurs when tail element of the dequeue contains link to …\nOccurs when while searching pre-tail, element wasn’t …\nOccurs when tail should contain value, but it’s empty …\nOccurs when tail should be empty, but it contains value …\nGets value stored under given keys, if present, and …\nGets value stored under given key, if present, and removes …\nGets value stored under given keys, if present, and …\nGets stored value, if present, and removes it from storage.\nStored value of current node.")