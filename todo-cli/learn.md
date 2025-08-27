1. 了解了rust里enum的底层实现：即如
    ```rust
    #[derive(ValueEnum, Clone)]
    enum Operation {
        Add,
        Remove,
        List,
        Quit,
        Mark,
    }
    ```
    如果没有derive宏，本质上在内存里，就是分配的递增的整数值1，2，3，4，5
    而derive宏ValueEnum，就是让他们实现了clap里的ValueEnum的trait
    在编译的时候，会生成以下代码，实现命令行的参数转换
    ```rust
    // 这是宏为你生成的，你不需要手动写！
    impl clap::ValueEnum for Operation {
        // 如何从字符串解析
        fn from_str(input: &str, ...) -> Result<Self, ...> {
            match input {
                "add" => Ok(Operation::Add),
                "remove" => Ok(Operation::Remove),
                "list" => Ok(Operation::List),
                "quit" => Ok(Operation::Quit),
                "mark" => Ok(Operation::Mark),
                _ => Err(/* ...一些错误信息... */),
            }
        }

        // 如何生成所有可能值的列表
        fn value_variants<'a>() -> &'a [Self] {
            &[Operation::Add, Operation::Remove, Operation::List, Operation::Quit, Operation::Mark]
        }
    }
    ```

2. 学习了clap的使用，这里的short指参数可以用单个-，long指的是参数可以用--，value_parser则是自动解析，把用户的命令行参数转化为rust需要的类型
    ```rust
    #[derive(Parser)]
    struct Args {
        #[clap(short, long, value_parser)]
        operation: Operation,
        //可选参数
        #[clap(short, long, value_parser)]
        content: Option<String>,
        //可选参数
        #[clap(short, long, value_parser)]
        id: Option<u32>,
    }
    ```

3. 学习了一些serde_json的应用，使用需要在对应的结构体上加**#[derive(Serialize, Deserialize)]**宏
    具体使用方法则是例如
    ```rust
    // ✅ 结构体 -> JSON
    let user = User { id: 1, name: "Alice".into(), active: true };
    let json_str = serde_json::to_string(&user).unwrap();
    println!("序列化: {}", json_str);

    // ✅ JSON -> 结构体
    let parsed: User = serde_json::from_str(&json_str).unwrap();
    println!("反序列化: {:?}", parsed);
    ```
    to_writer_pretty可以写出漂亮的json形式

4. 了解了File和BufWriter的文件方面的包
    ```rust
    fn remove_todo(id: u32) -> Result<(), serde_json::Error> {
        let mut todos = load_todos()?;
        //retain用于过滤掉不满足条件的元素，这里过滤掉id不等于id的元素
        todos.retain(|todo| todo.id != id);
        //File::create用于创建文件，如果文件不存在，则创建文件，如果文件存在，则覆盖文件
        let file = File::create("data/todo.json").unwrap();
        //BufWriter是用于缓冲写入的，可以提高写入效率
        //和普通写入的区别是，普通写入会直接写入文件，而BufWriter会先写入缓冲区，然后一次性写入文件，这样可以提高写入效率
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &todos)?;
        Ok(())
    }
    ```