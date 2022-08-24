
https://matt.might.net/articles/by-example-continuation-passing-style/

Implementing exceptions in CPS

~~~js
function fact (n) {
  if (n < 0)
    throw "n < 0" ;
  else if (n == 0)
    return 1 ;
  else
    return n * fact(n-1) ;
}
 
function total_fact (n) {
  try {
    return fact(n) ;
  } catch (ex) {
    return false ;
  }
}
~~~

By adding an exceptional continuation in CPS, we can desugar the throw, try and catch:

~~~js
function fact (n,ret,thro) {
 if (n < 0)
   thro("n < 0") 
 else if (n == 0)
   ret(1)
 else
   fact(n-1,
        function (t0) {
          ret(n*t0) ;
        },
        thro)
}
 
function total_fact (n,ret) {
  fact (n,ret,
    function (ex) {
      ret(false) ;
    }) ;
}
 
total_fact(10, function (res) {
  document.write("total_fact(10): " + res)
}) ;
 
total_fact(-1, function (res) {
  document.write("total_fact(-1): " + res)
}) ;

~~~