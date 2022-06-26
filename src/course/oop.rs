//使用trait对象存储不同类型的值
trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

struct ScreenGenric<T>
    where T: Draw {
    components: Vec<T>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

impl<T: Draw> ScreenGenric<T> {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {
    width: usize,
    height: usize,
    content: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button[{}x{}], content: {}.", self.width, self.height, self.content);
    }
}

struct SelectedRadio {
    options: Vec<String>,
}

impl Draw for SelectedRadio {
    fn draw(&self) {
        println!("draw selected radio: {:?}", self.options)
    }
}

#[test]
fn test_screen() {
    let screen = Screen {
        components: vec![
            Box::new(
                Button {
                    width: 10,
                    height: 2,
                    content: String::from("ok"),
                }
            ),
            Box::new(
                SelectedRadio {
                    options: vec!["summer".to_string(), "spring".to_string()],
                }
            ),
        ]
    };
    screen.run();
}

#[test]
fn test_screen_genric() {
    ScreenGenric{
        components: vec![
            Button {
                width: 10,
                height: 2,
                content: String::from("ok"),
            },
            /*SelectedRadio {
                options: vec!["summer".to_string(), "spring".to_string()],
            }*/
        ]
    };
}

//通过单态化生成的代码会执行静态派发，这意味着编译器能够在编译过程中确定你调用的具体方法
//动态派发，编译器在编译过程中无法确定你调用的究竟是哪一个方法，在进行动态派发的场景中，编译器会
//生成一些额外的代码以便在运行时找出我们希望调用的方法

//trait对象必须保证对象安全
//1、方法的返回类型不是Self;
//2、方法中不包含任何泛型参数