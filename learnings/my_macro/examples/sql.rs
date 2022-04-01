use my_macro::sql;


fn main() {
    sql!( select * from users where id = 1 and age>20 order by crreated desc limit 10 );

}