mod clean_arch {
    // @see
    pub struct CreateUserInput {
        // ...
    }
    pub struct CreateUserOutput {
        // ...
    }
    pub trait UserCrudPort {
        fn create(
            &self,
            input: CreateUserInput,
        ) -> Result<CreateUserOutput, Box<dyn std::error::Error>>;
        // TODO design the crud method signatures ...
        fn update(&self);
        fn delete(&self);
        fn query(&self);
    }

    pub trait HaveUserCrudPort {
        type CrudPort: UserCrudPort;

        /// .
        fn provide_user_crud_port(&self) -> &Self::CrudPort;
    }
}

mod dep_attr {
    // As a comment, there are some RFCs to allow fields on traits
    // https://stackoverflow.com/questions/68171921/require-a-field-as-part-of-a-rust-trait
    // https://stackoverflow.com/questions/65380698/trait-with-default-implementation-and-required-struct-member

    pub struct SomeType {
        // some data...
    }

    pub trait FooTrait {
        // Traits can't provide or require struct fields. Though there is an RFC (#1546) about allowing fields in traits.
        fn get_some_attr(&self) -> &SomeType;

        fn foo(&self) -> Result<String, String> {
            let _myDep = self.get_some_attr();
            // do some other stuff

            Ok("ok".to_string())
        }
    }

    // ## 2
    trait Jobs {
        fn add_job(&mut self, job: String);
        fn clear_jobs(&mut self);
        fn count_jobs(&self) -> usize;
    }

    macro_rules! impl_jobs_with_field {
        ($($t:ty),+ $(,)?) => ($(
            impl Jobs for $t {
                fn add_job(&mut self, job: String) {
                    self.jobs.push(job);
                }

                fn clear_jobs(&mut self) {
                    self.jobs.clear();
                }

                fn count_jobs(&self) -> usize {
                    self.jobs.len()
                }
            }
        )+)
    }

    struct Person {
        jobs: Vec<String>,
    }

    struct Customer {
        jobs: Vec<String>,
    }

    impl_jobs_with_field!(Person);
    impl_jobs_with_field!(Customer);
    // or
    // impl_jobs_with_field!(Person, Customer);

    mod blanet_impls {
        use super::Jobs;

        trait HasJobs {
            fn jobs(&self) -> &[String];
            fn jobs_mut(&mut self) -> &mut Vec<String>;
        }

        impl<T: HasJobs> Jobs for T {
            fn add_job(&mut self, job: String) {
                self.jobs_mut().push(job);
            }

            fn clear_jobs(&mut self) {
                self.jobs_mut().clear();
            }

            fn count_jobs(&self) -> usize {
                self.jobs().len()
            }
        }

        macro_rules! impl_has_jobs {
            ($($t:ty),+ $(,)?) => ($(
                impl HasJobs for $t {
                    fn jobs(&self) -> &[String] {
                        &self.jobs
                    }

                    fn jobs_mut(&mut self) -> &mut Vec<String> {
                        &mut self.jobs
                    }
                }
            )+)
        }

        struct Person {
            jobs: Vec<String>,
        }

        struct Customer {
            jobs: Vec<String>,
        }

        impl_has_jobs!(Person);
        impl_has_jobs!(Customer);
        // or
        // impl_has_jobs!(Person, Customer);
    }

    mod derefs {
        struct Person {
            jobs: Vec<String>,
        }

        impl Jobs for Person {
            fn add_job(&mut self, job: String) {
                self.jobs.push(job);
            }

            fn clear_jobs(&mut self) {
                self.jobs.clear();
            }

            fn count_jobs(&self) -> usize {
                self.jobs.len()
            }
        }

        use std::ops::{Deref, DerefMut};

        use super::Jobs;

        struct Customer {
            person: Person,
        }

        impl Deref for Customer {
            type Target = Person;

            fn deref(&self) -> &Self::Target {
                &self.person
            }
        }

        impl DerefMut for Customer {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.person
            }
        }
    }
}
