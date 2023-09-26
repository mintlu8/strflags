
use strflags::*;

str_flags! {
    Animal: [
        #[doc="dog"]
        Dog,
        /// cat
        Cat,
        /// Rabbit
        Rabbit,
        Giraffe,
        Whale,
        Dolphin,
    ]
}



str_enum! {
    VeryLarge : [
        // This is fine as long as we don't use it
        NNopjejqiewjqvckqnvkoqpvjqpcqkc
    ]
}

#[test]
fn stringify() {
    assert_eq!(Animal::Dog, "dog");
    assert_ne!(Animal::Dog, "Dog");
    assert_ne!(Animal::Dog, "cat");
}

#[test]
fn init_ops() {
    type F = Flags<Animal, '|'>;

    assert!(F::EMPTY.is_none());
    assert!(!F::EMPTY.is_some());
    assert!(F::EMPTY.len() == 0);

    let mut one = F::new(Animal::Dog);
    assert!(one.is_some());
    assert!(one.len() == 1);
    assert!(one.contains(Animal::Dog));
    assert!(!one.contains(Animal::Cat));
    
    one += Animal::Cat;
    assert!(one.len() == 2);
    one += Animal::Cat;
    assert!(one.len() == 2);
    one += Animal::Rabbit;
    assert!(one.len() == 3);
    one += Animal::Rabbit;
    assert!(one.len() == 3);
    assert!(one.contains(Animal::Cat));
    assert!(one.contains(Animal::Rabbit));

    let same = F::pair(Animal::Dog, Animal::Dog);
    assert!(same.len() == 1);

    let pair = F::pair(Animal::Dog, Animal::Cat);
    assert!(pair.len()==2);
    assert!(pair.contains(Animal::Dog));
    assert!(pair.contains(Animal::Cat));
    assert!(!pair.contains(Animal::Giraffe));
}

#[test]
fn or() {
    let animals1 = Animal::Dog | Animal::Cat | Animal::Giraffe;
    let animals2 = Animal::Rabbit | Animal::Whale | Animal::Giraffe;
    assert!(animals1.len() == 3);
    assert!(animals2.len() == 3);
    assert!((animals1 | animals2).len() == 5);
}

#[test]
fn and() {
    let mut animals = Animal::Giraffe | Animal:: Whale | Animal::Rabbit;
    assert!(animals.contains(Animal::Giraffe));
    assert!(animals.contains(Animal::Whale));
    assert!(animals.contains(Animal::Rabbit));
    let a = animals.clone() & Animal::Rabbit;
    assert!(!a.contains(Animal::Giraffe));
    assert!(a.contains(Animal::Rabbit));
    assert!(!a.contains(Animal::Whale));
    animals &= Animal::Dog;
    assert!(!animals.contains(Animal::Dog));
    assert!(!animals.contains(Animal::Whale));
    assert!(!animals.contains(Animal::Rabbit));

    let animals1 = Animal::Dog | Animal::Cat | Animal::Giraffe;
    let animals2 = Animal::Dog | Animal::Cat | Animal::Whale;
    let intersect = animals1 & animals2;
    assert!(intersect.len() == 2);
    assert!(intersect.contains(Animal::Dog));
    assert!(intersect.contains(Animal::Cat));
}

#[test]
fn xor() {
    let mut animals = Animal::Dog | Animal:: Dolphin | Animal::Cat;
    assert!(animals.contains(Animal::Dog));
    assert!(animals.contains(Animal::Dolphin));
    assert!(animals.contains(Animal::Cat));
    let a = animals.clone() ^ Animal::Dog;
    assert!(!a.contains(Animal::Dog));
    assert!(a.contains(Animal::Dolphin));
    assert!(a.contains(Animal::Cat));
    animals ^= Animal::Dog;
    assert!(!animals.contains(Animal::Dog));
    assert!(animals.contains(Animal::Dolphin));
    assert!(animals.contains(Animal::Cat));

    let animals1 = Animal::Dog | Animal::Cat | Animal::Giraffe;
    let animals2 = Animal::Dog | Animal::Cat | Animal::Whale;
    let intersect = animals1 ^ animals2;
    assert!(intersect.len() == 2);
    assert!(intersect.contains(Animal::Giraffe));
    assert!(intersect.contains(Animal::Whale));
}


#[test]
fn sub() {
    let mut animals = Animal::Dog | Animal:: Dolphin | Animal::Cat;
    assert!(animals.contains(Animal::Dog));
    assert!(animals.contains(Animal::Dolphin));
    assert!(animals.contains(Animal::Cat));
    animals -= Animal::Dog;
    assert!(animals.len() == 2);
    animals -= Animal::Dog;
    assert!(animals.len() == 2);
    animals -= Animal::Dolphin;
    assert!(animals.len() == 1);
    assert!(animals.contains(Animal::Cat));

    let animals1 = Animal::Dog | Animal::Cat | Animal::Giraffe;
    let animals2 = Animal::Dog | Animal::Cat | Animal::Whale;
    let diff = animals1 - animals2;
    assert!(diff.len() == 1);
    assert!(diff.contains(Animal::Giraffe));
}


str_flags! {
    #[derive(Default, PartialOrd, Ord)]
    pub Language: [
        /// Crab
        Rust,
        /// C++
        CPlusPlus
    ]
}

#[test]
fn test_v3() {
    assert!(Language::default() == "");
    assert!(Language::Rust > Language::CPlusPlus);
}