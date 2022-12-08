use std::cell::RefCell;
use std::rc::{Rc, Weak};

const TOT_DISK_SPACE: usize = 70000000;
const REQUIRED_UPDATE_SIZE: usize = 30000000;
const FILE_SIZE_LIMIT: usize = 100000;

pub fn solve() {
    let content: Vec<Vec<char>> = include_str!("../inputs/day07.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let base = Rc::new(Dir::new("/".to_string(), &RefCell::default()));

    let curr_dir: RefCell<Weak<Dir>> = RefCell::new(Rc::downgrade(&base));

    for line in &content {
        if line[0] == '$' {
            if line[2] == 'l' {
                // $ ls
                // do nothing, just handle its input when it comes
            }
            // now the command must be cd, in the form of:
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
                // move into a dir, assume it already has been displayed by ls,
                // i.e. is trackd in the fs
                let name: String = line[5..].iter().collect();
                let temp_dir: Rc<Dir> = (*curr_dir.borrow()).upgrade().unwrap();
                let nu_vec: &Vec<Rc<Dir>> = &*temp_dir.child_dirs.borrow();
                let found_dir: &Rc<Dir> = nu_vec.iter().find(|x| x.name == name).unwrap();

                curr_dir.replace(Rc::downgrade(found_dir));
            }
        } else if line[0] == 'd' {
            // d for dir
            // adds the item into our fs, we assume duplicates don't happen,
            // i.e. ls is never called twice
            //
            // format:
            // dir 12..
            // 01234
            // name starts at 4
            let name = line[4..].iter().collect();

            let nu_dir = Rc::new(Dir::new(name, &curr_dir));
            // Breakdown of how adding works:

            // curr_dir is a RefCell<Weak<Dir>> (the parent Type)
            curr_dir
                .borrow() // we get a Ref<Weak<Dir>>, which we can operate on
                .upgrade()
                // upgrade the Weak ref into an Rc, so we can operate on it
                //  from the docs: it delays dropping of the inner value if successful
                //  basically, while we're using it, don't destroy it
                .unwrap()
                // if the thing it's pointing is destroyed, then we get None
                // however, with our DS, this doesn't happen. so it always returns the val.
                .child_dirs
                .borrow_mut() // we need to borrow_mut to mutate the child_dirs
                .push(nu_dir);
        } else {
            // file stuff
            // the first thing is a number which indicates file size, so just go for it.
            // 57426 gszshjwr.lrs

            // find the index of the space and split at it.
            let ind = line.iter().position(|chr| *chr == ' ').unwrap();
            let (size, name) = line.split_at(ind);

            let name = name[1..].iter().collect();
            let size = size.iter().collect::<String>().parse::<usize>().unwrap();
            curr_dir
                .borrow()
                .upgrade()
                .unwrap()
                .direct_files
                .borrow_mut()
                .push(Rc::new(File { name, size }))
        }
    }
    let mut size_of_files_over_limit = 0;
    let curr_size = base.tot_size(&mut size_of_files_over_limit);
    let mut del_target_size = curr_size;

    base.target_deletion(curr_size, &mut del_target_size);
    println!("Part 1: {size_of_files_over_limit}");
    println!("Part 2: {del_target_size}");
}

// We need these to be RefCell to enable interior mutability.
// Why? Well when we ascend the fs with `cd ..`, we will be mutating
// the "curr_dir" tracker. So it needs access to the internals.
// I.e. it allows us to navigate the fs.
// we can't borrow data out of an RC NOTE
#[derive(Debug, Clone)]
struct Dir {
    name: String,
    parent: RefCell<Weak<Dir>>,
    child_dirs: RefCell<Vec<Rc<Dir>>>,
    direct_files: RefCell<Vec<Rc<File>>>,
}

/// to prevent dups, why not use a map..?
/// because we can't. the parent pointer isn't hashable
/// we could try working off names
/// but.. then we'd need full paths. (i think)
/// also, dup detection isn't necessary because the input doesn't try to cd
/// into directories that have not yet been revealed ls

impl Dir {
    fn new(name: String, parent: &RefCell<Weak<Dir>>) -> Self {
        Dir {
            name,
            parent: RefCell::clone(parent),
            child_dirs: RefCell::new(Vec::new()),
            direct_files: RefCell::new(Vec::new()),
        }
    }

    /// Actually solving the problem:
    /// these two functions do the same thing (see size of dirs) and both do it
    /// in a bad + weird way: passing around a pointer that gets mutated
    /// TODO maybe store fize in the directory itself, so that accessing is O(1)?

    /// Find the sum of all directories under the FILE_SIZE_LIMIT
    fn tot_size(&self, track_sum: &mut usize) -> usize {
        let files = &*self.direct_files.borrow();
        let dirs = &*self.child_dirs.borrow();
        let sum = files.iter().fold(0, |acc, file| file.size + acc)
            + dirs
                .iter()
                .map(|child: &Rc<Dir>| child.tot_size(track_sum))
                .sum::<usize>();
        if sum <= FILE_SIZE_LIMIT {
            *track_sum += sum;
        }
        sum
    }

    /// we need to have REQUIRE_UPDATE_SIZE left available to update,
    /// so we want to find the smallest
    /// directory to delete that lets us do this
    fn target_deletion(&self, tot_size: usize, del_target_size: &mut usize) -> usize {
        let files = &*self.direct_files.borrow();
        let dirs = &*self.child_dirs.borrow();
        let sum = files.iter().fold(0, |acc, file| file.size + acc)
            + dirs
                .iter()
                .map(|child| child.target_deletion(tot_size, del_target_size))
                .sum::<usize>();

        if TOT_DISK_SPACE - REQUIRED_UPDATE_SIZE >= (tot_size - sum) {
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
    size: usize,
}
