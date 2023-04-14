/*

trait AsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

trait AsMut<T: ?Sized> {
    fn as_mut(&mut self) -> &mut T;
}

AsRef is for cheap reference to reference conversions between semantically equivalent things


 */

mod usage {
    // accepts:
    //  - &str
    //  - &String
    fn takes_str(s: &str) {
        // use &str
    }

    // accepts:
    //  - &str
    //  - &String
    //  - String
    fn takes_asref_str<S: AsRef<str>>(s: S) {
        let s: &str = s.as_ref();
        // use &str
    }

    fn example(slice: &str, borrow: &String, owned: String) {
        takes_str(slice);
        takes_str(borrow);
        // takes_str(owned); // ❌
        takes_asref_str(slice);
        takes_asref_str(borrow);
        takes_asref_str(owned); // ✅
    }
}

mod bad_example {
    struct User {
        // name: String,
        // age: u32,
        name: String,
        email: String,
        age: u32,
        height: u32,
    }

    impl AsRef<String> for User {
        fn as_ref(&self) -> &String {
            // uh, do we return name or email here?
            &self.name
        }
    }

    impl AsRef<u32> for User {
        fn as_ref(&self) -> &u32 {
            //   // uh, do we return age or height here?
            &self.age
        }
    }
}

mod good_example {
    struct User {
        name: String,
        age: u32,
    }

    // unfortunately the standard library cannot provide
    // a generic blanket impl to save us from this boilerplate
    impl AsRef<User> for User {
        fn as_ref(&self) -> &User {
            self
        }
    }

    enum Privilege {
        BanUsers,
        EditPosts,
        DeletePosts,
    }

    // although Moderators have some special
    // privileges they are still regular Users
    // and should be able to do all the same stuff
    struct Moderator {
        user: User,
        privileges: Vec<Privilege>,
    }

    impl AsRef<Moderator> for Moderator {
        fn as_ref(&self) -> &Moderator {
            self
        }
    }

    impl AsRef<User> for Moderator {
        fn as_ref(&self) -> &User {
            &self.user
        }
    }

    // this should be callable with Users
    // and Moderators (who are also Users)
    fn create_post<U: AsRef<User>>(u: U) {
        let user = u.as_ref();
        // etc
    }

    fn example(user: User, moderator: Moderator) {
        create_post(&user);
        create_post(&moderator); // ✅
    }
}

mod explicit_types_conversion {
    use std::convert::AsRef;

    struct Human {
        health_points: u32,
    }

    impl AsRef<Human> for Human {
        fn as_ref(&self) -> &Human {
            self
        }
    }

    enum Weapon {
        Spear,
        Axe,
        Sword,
    }

    // a Soldier is just a Human with a Weapon
    struct Soldier {
        human: Human,
        weapon: Weapon,
    }

    impl AsRef<Soldier> for Soldier {
        fn as_ref(&self) -> &Soldier {
            self
        }
    }

    impl AsRef<Human> for Soldier {
        fn as_ref(&self) -> &Human {
            &self.human
        }
    }

    enum Mount {
        Horse,
        Donkey,
        Cow,
    }

    // a Knight is just a Soldier with a Mount
    struct Knight {
        soldier: Soldier,
        mount: Mount,
    }

    impl AsRef<Knight> for Knight {
        fn as_ref(&self) -> &Knight {
            self
        }
    }

    impl AsRef<Soldier> for Knight {
        fn as_ref(&self) -> &Soldier {
            &self.soldier
        }
    }

    impl AsRef<Human> for Knight {
        fn as_ref(&self) -> &Human {
            &self.soldier.human
        }
    }

    enum Spell {
        MagicMissile,
        FireBolt,
        ThornWhip,
    }

    // a Mage is just a Human who can cast Spells
    struct Mage {
        human: Human,
        spells: Vec<Spell>,
    }

    impl AsRef<Mage> for Mage {
        fn as_ref(&self) -> &Mage {
            self
        }
    }

    impl AsRef<Human> for Mage {
        fn as_ref(&self) -> &Human {
            &self.human
        }
    }

    enum Staff {
        Wooden,
        Metallic,
        Plastic,
    }

    // a Wizard is just a Mage with a Staff
    struct Wizard {
        mage: Mage,
        staff: Staff,
    }

    impl AsRef<Wizard> for Wizard {
        fn as_ref(&self) -> &Wizard {
            self
        }
    }

    impl AsRef<Mage> for Wizard {
        fn as_ref(&self) -> &Mage {
            &self.mage
        }
    }

    impl AsRef<Human> for Wizard {
        fn as_ref(&self) -> &Human {
            &self.mage.human
        }
    }

    fn borrows_human<H: AsRef<Human>>(human: H) {}
    fn borrows_soldier<S: AsRef<Soldier>>(soldier: S) {}
    fn borrows_knight<K: AsRef<Knight>>(knight: K) {}
    fn borrows_mage<M: AsRef<Mage>>(mage: M) {}
    fn borrows_wizard<W: AsRef<Wizard>>(wizard: W) {}

    fn example(human: Human, soldier: Soldier, knight: Knight, mage: Mage, wizard: Wizard) {
        // all types can be used as Humans
        borrows_human(&human);
        borrows_human(&soldier);
        borrows_human(&knight);
        borrows_human(&mage);
        borrows_human(&wizard);
        // Knights can be used as Soldiers
        borrows_soldier(&soldier);
        borrows_soldier(&knight);
        // Wizards can be used as Mages
        borrows_mage(&mage);
        borrows_mage(&wizard);
        // Knights & Wizards passed as themselves
        borrows_knight(&knight);
        borrows_wizard(&wizard);
    }
}
