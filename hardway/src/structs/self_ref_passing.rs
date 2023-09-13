// https://stackoverflow.com/questions/30681468/passing-mutable-self-reference-to-method-of-owned-object
// https://stackoverflow.com/questions/59364133/is-it-possible-to-call-a-parent-structs-methods-from-a-child-struct
// ⚠️ 很推荐看👀上面👆的两篇问答！

mod remove_member_from_self {
    use std::mem;

    #[derive(Default)]
    struct Ball {
        size: u8,
    }

    impl Ball {
        fn update(&mut self, field: &Field) {}
    }

    struct Field {
        ball: Ball,
    }

    impl Field {
        fn update(&mut self) {
            let mut ball = mem::replace(&mut self.ball, Ball::default());
            ball.update(self);
            self.ball = ball;
        }
    }
}

mod option_and_take {
    struct Ball {
        size: u8,
    }

    impl Ball {
        fn update(&mut self, field: &Field) {}
    }

    struct Field {
        ball: Option<Ball>,
    }

    impl Field {
        fn update(&mut self) {
            if let Some(mut ball) = self.ball.take() {
                ball.update(self);
                self.ball = Some(ball);
            }
        }
    }
}

mod runtime_checks {
    use std::cell::RefCell;

    struct Ball {
        size: u8,
    }

    impl Ball {
        fn update(&mut self, field: &Field) {}
    }

    struct Field {
        ball: RefCell<Ball>,
    }

    impl Field {
        fn update(&mut self) {
            self.ball.borrow_mut().update(self)
        }
    }
}

mod use_needed_field {
    struct Ball {
        size: u8,
    }

    impl Ball {
        fn update(&mut self, field: &u8) {}
    }

    struct Field {
        players: u8,
        ball: Ball,
    }

    impl Field {
        fn update(&mut self) {
            self.ball.update(&self.players)
        }
    }

    mod _1{
        struct Ball {
            size: u8,
        }
        
        impl Ball {
            fn update(&mut self, field: BallUpdateInfo) {}
        }
        
        struct BallUpdateInfo<'a> {
            players: &'a u8,
        }
        
        struct Field {
            players: u8,
            ball: Ball,
        }
        
        impl Field {
            fn update(&mut self) {
                let info = BallUpdateInfo { players: &self.players };
                self.ball.update(info)
            }
        }
    }

    mod _2{
        struct Ball {
            size: u8,
        }
        
        impl Ball {
            fn update(&mut self, field: &UpdateInfo) {}
        }
        
        struct UpdateInfo {
            players: u8,
        }
        
        struct Field {
            update_info: UpdateInfo,
            ball: Ball,
        }
        
        impl Field {
            fn update(&mut self) {
                self.ball.update(&self.update_info)
            }
        }
        
    }
}
