IN = (1+x)^30;

{
    expand;
}

{
    id x^n? = f(n?);
    repeat id f(x?{>0},?a) = f(x? - 1,?a) + f(x? - 2,?a);
}
