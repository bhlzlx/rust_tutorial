# 关于生命周期的一个问题

## 协变 逆变 不变

## 一个简单的例子

```rust
#[derive(Debug)]
struct Number<'a> { 
    num: &'a u8 
}
impl<'a> Number<'a> { 
    fn get_num(&self) -> &'a u8 { 
        self.num 
    }  
    fn set_num(&mut self, new_number: &'a u8) { 
        self.num = new_number 
    }
}
```

`struct Number<'a>` 中的`'a` 代表着这个类型实例的生命周期参数，而`num`的类型`&'a u8`在生命周期`'a`这个意义上来讲，通过查阅表格会发现，`&'a`是`'a`的协变，可认为是`'a`的子类型，那就意味着`num`的生命周期绝对要比`'a`要久，也意味着这个成员的引用的生命周期一定要比`Number`对象久！


```rust
fn evil_feeder<'a,>(input: &'a mut T, val: T) {
    *input = val;
}

fn main() {
    let mut mr_snuggles: &'static str = "meow! :3";  // mr. snuggles forever!!
    {
        let spike = String::from("bark! >:V");
        let spike_str: &str = &spike;                // Only lives for the block
        // let static_str: &'static str = "hello,world!";
        evil_feeder(&mut mr_snuggles, spike_str);    // EVIL!
    }
    println!("{}", mr_snuggles);                     // Use after free?
}
```

函数体声明的生命周期参数只针对函数体内的数据进行检查，