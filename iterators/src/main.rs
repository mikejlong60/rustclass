fn main() {
    //0 .. 10 below is a range. That's one kind of iterator. The other is iter().
    for x in 0 .. 10 {
        println!("Hello, {}", x);
    }

    let nums = vec![1,2,3,4,5,6];
    for x in &nums {
        println!("Hello Vector1, {}", *x);
    }


    let nums2 = (1..100).collect::<Vec<_>>();
    //collect takes as many values as the iterator will give it and turns them into a collection.
    for x in &nums2 {
        println!("Hello Vector2, {}", *x);
    }

    //find takes as many values from an iterator as meet the predicate lambda.  It returns the first one it finds.
    //It returns an option because it might not find any, the standard Option monad you're used to using in Scala.
    let greater_than_forty = (1..100).find(|x| *x > 40);
    //let greater_than_forty2 = nums2.find(|x| x > 40);
    for x in greater_than_forty {
        println!("Hello find, {}", x);
    }

    //fold works just like Scala.  It takes a base, an accumulator, and a lambda.
    let sumOf1To100 = (1..100).fold(0, |sum, x| sum + x);
    println!("sum 1 to 100:, {}", sumOf1To100);

    //Iterators are lazy like in Haskell.  That is really important.  The iter() kind of iterator
    //can turn a vector into an iterator.
    let nums2 = vec![1,2,3,4,5,6];
    for x in nums2.iter() {
        println!("Hello Vector3, {}", *x);
    }


    //map is lazy too. won't do anything unless you use it.
    let fred = (1..1000).take(200).map(|x| x + 12);
    for x in fred {
        println!("first 200 with 12 added:, {}",x);
    }

    //here is an infinite iterator
    let fred2 = (1..).take(10).map(|x| x + 12);
    for x in fred2 {
        println!("first 10 of infinite iterator with 12 added:, {}",x);
    }

    //here is filter. takes a closure predicate.  still lazy so it won't print anything.
    let fred3 = (1..).take(20).filter(|x| x % 2 == 0);

    //you can use the & which actually retrieves the value from the reference.
    for i in (1..).take(2000).filter(|&x| x % 13 == 0) {
        println!("1, {}", i);
    }

}
