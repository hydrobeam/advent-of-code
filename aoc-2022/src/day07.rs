use std::cell::RefCell;
use std::rc::{Rc, Weak};

const TOT_DISK_SPACE: isize = 70000000;
const REQUIRED_UPDATE_SIZE: isize = 30000000;

pub fn solve() {
    let content: Vec<Vec<char>> = include_str!("../inputs/day07.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let base = Rc::new(Dir {
        name: "/".to_string(),
        parent: RefCell::new(Weak::new()),
        child_dirs: RefCell::new(Vec::new()),
        direct_files: RefCell::new(Vec::new()),
    });

    let curr_dir: RefCell<Weak<Dir>> = RefCell::new(Rc::downgrade(&base));

    for line in &content {
        if line[0] == '$' {
            if line[2] == 'l' {
            }
            // skip

            // now:
            // must be cd, in the form of:
            // $ cd name...
            // 012345
            // first index of name is 5
            else if line[5] == '.' {
                // $ cd ..
                // go upsies
                let bruh = &curr_dir.borrow().upgrade().unwrap().parent;
                let bruh_2 = Weak::clone(&*bruh.borrow());
                // * to turn the Ref<Weak<Dir>> (after the borrow) into a Weak<Dir>
                // & to turn the Weak<Dir> into a &Weak<dir> (needed for clone)
                curr_dir.replace(bruh_2);
                continue;
            } else if line[5] == '/' {
                // back to base
                curr_dir.replace(Rc::downgrade(&base));
            } else {
                // $ cd name..
                // we move into a dir
                let name: String = line[5..].iter().collect();
                let nu_dir = Rc::new(Dir {
                    name: name.clone(),
                    parent: RefCell::clone(&curr_dir),
                    child_dirs: RefCell::new(Vec::new()),
                    direct_files: RefCell::new(Vec::new()),
                });
                // we assume dups don't happen:
                {
                    Weak::upgrade(&*curr_dir.borrow())
                        .unwrap()
                        .child_dirs
                        .borrow_mut()
                        .push(Rc::clone(&nu_dir));
                }

                curr_dir.replace(Rc::downgrade(&nu_dir));
            }
        } else if line[0] == 'd' {
            // d for dir
            // dir 12..
            // 01234
            // name starts at 4
            let name = line[4..].iter().collect();

            let nu_dir = Rc::new(Dir {
                name,
                parent: RefCell::clone(&curr_dir),
                child_dirs: RefCell::new(Vec::new()),
                direct_files: RefCell::new(Vec::new()),
            });
            Weak::upgrade(&*curr_dir.borrow())
                .unwrap()
                .child_dirs
                .borrow_mut()
                .push(nu_dir);
        } else {
            // file stuff
            // the first thing is a number which indicates file size, so just go for it.
            // 57426 gszshjwr.lrs
            // TODO make this less bad
            //
            let mut ind = 0;
            while line[ind] != ' ' {
                ind += 1;
            }
            let (size, name) = line.split_at(ind);

            Weak::upgrade(&*curr_dir.borrow_mut())
                .unwrap()
                .direct_files
                .borrow_mut()
                .push(Rc::new(File {
                    // split_at includes the splitting character for the right side,
                    // so adjust by 1
                    name: name[1..].iter().collect(),
                    size: size.iter().collect::<String>().parse::<isize>().unwrap(),
                }))
        }

        // dbg!(&base);
    }
    let mut var = 0;
    let curr_size = base.tot_size(&mut var);
    let mut del_target = curr_size;

    println!("Part 1: {var}");
    base.target_deletion(curr_size, &mut del_target);
    println!("Part 2: {del_target}");
}

// We need these to be RefCell to enable interior mutability.
// Why? Well when we ascend the fs with `cd ..`, we will be mutating
// the "curr_dir" tracker. So it needs access to the internals.
// I.e. it allows us to navigate the fs.
#[derive(Debug, Clone)]
struct Dir {
    name: String,
    parent: RefCell<Weak<Dir>>,
    child_dirs: RefCell<Vec<Rc<Dir>>>,
    direct_files: RefCell<Vec<Rc<File>>>,
}

// to prevent dups, why not use a map..?
// because we can't. the parent pointer isn't hashable
// we could try working off names
// but.. then we'd need full paths. (i think)
// also, dup detection isn't necessary because the input doesn't try to cd
// into directories that have not yet been revealed ls

impl Dir {
    fn tot_size(&self, track_sum: &mut isize) -> isize {
        let swag = &*self.direct_files.borrow();
        // dbg!(&swag);
        let swag_2 = &*self.child_dirs.borrow();
        let sum = swag.iter().fold(0, |acc, file| file.size + acc)
            + swag_2
                .iter()
                // .inspect(|x| println!("mapping {}", x.name))
                .map(|child| child.tot_size(track_sum))
                .sum::<isize>();
        // dbg!(sum)

        // dbg!(&self.name, sum);
        if sum <= 100000 {
            *track_sum += sum;
            // println!("{sum}");
        }
        sum
    }

    fn target_deletion(&self, tot_size: isize, del_target_size: &mut isize) -> isize {
        let swag = &*self.direct_files.borrow();
        // dbg!(&swag);
        let swag_2 = &*self.child_dirs.borrow();
        let sum = swag.iter().fold(0, |acc, file| file.size + acc)
            + swag_2
                .iter()
                // .inspect(|x| println!("mapping {}", x.name))
                .map(|child| child.target_deletion(tot_size, del_target_size))
                .sum::<isize>();
        if TOT_DISK_SPACE - (tot_size - sum) >= REQUIRED_UPDATE_SIZE {
            if sum <= *del_target_size {
                *del_target_size = sum;
            }
        }
        sum
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: isize,
}

// trait Command {
//     fn run();
// }
