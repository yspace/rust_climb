When to use lifetimes annotations

 lifetimes annotations should never surface in any public API. It’s okay to use them if you need absolute performance AND minimal re- sources usage AND are doing embedded development, but you should keep them hidden in your code, and they should never surface in the public API.

// TODO 翻译下