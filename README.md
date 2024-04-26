# wit-for-process-apis-experiments

These are some experiments to understand how to use [WIT files](https://component-model.bytecodealliance.org/) for process APIs in [Kinode OS](https://github.com/kinode-dao/kinode).
We already use WIT files for our processes -- which are Wasm components.
These experiements are to see if we can define libraries using WIT/Wasm, where the WIT file takes on the form of a header file and the Wasm file takes on the form of a `.so` file.

## Some important links

https://component-model.bytecodealliance.org/

https://gist.github.com/nick1udwig/060e581c160d17d41d30dfe331949e9d

https://gist.github.com/nick1udwig/4493c582a1f3dc08b3000186864e5bae

## Speculation on how to make these easy to use in [`kit`](https://github.com/kinode-dao/kit)

These are very much "personal note" level notes and this is still a WIP:


* can either write code to derive the wit file from eg rust code OR require that users write it
* it'd be nice to offer tools for deriving
* however users will have to read either wit file or smth derived from it to understand interface
* so either need templates or tools for deriving or both, but also discussion of HOW TO because users will need to write them

* build process should ideally detect the existence of api, build it and publish it along w package, and compose it; without any work from user
* are APIs for processes or packages?
* APIs in metadata.json or manifest.json or elsewhere?

* what about multiple deps, ie my process/package imports 2 apis?
* what about nested deps, ie foo deps on bar and baz deps on foo?
*

* api consists of one or two files:
   * wit
   * wasm
* Could do, e.g.:
   * api defined in metadata.json
   * api wit & wasm tar'd in pkg

* should we republish APIs we depend on? If we don't, we risk hosts going offline
* maybe we just require mirroring of APIs we consume

* is nesting real?
* do we actually ever want to export functions? Probably. Libs




* unrelated: kit tool for upgrading an existing process to current v?
* would look like a stepwise update from previous to current
