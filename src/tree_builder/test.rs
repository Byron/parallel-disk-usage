use crate::{
    size::Bytes,
    tree_builder::{Info, Tree, TreeBuilder},
};
use build_fs_tree::{dir, file, FileSystemTree};
use derive_more::From;
use pretty_assertions::assert_eq;

type SampleData = Bytes;
type SampleName = String;
const SAMPLE_SEPARATOR: char = '/';
const SAMPLE_DIR_SIZE: SampleData = Bytes::new(5);

#[derive(Debug, From)]
struct SampleTree(FileSystemTree<String, &'static str>);

const fn len(text: &str) -> SampleData {
    SampleData::new(text.len() as u64)
}

impl SampleTree {
    fn create_sample() -> Self {
        SampleTree::from(dir! {
            "flat" => dir! {
                "0" => file!("")
                "1" => file!("a")
                "2" => file!("ab")
                "3" => file!("abc")
            }
            "nested" => dir! {
                "0" => dir! {
                    "1" => file!("abcdef")
                }
            }
            "empty-dir" => dir! {}
        })
    }

    fn tree(&self, root: &'static str) -> Tree<SampleName, SampleData> {
        Tree::from(TreeBuilder {
            path: root.to_string(),
            name: root.to_string(),
            get_info: |path| {
                let path: Vec<_> = path
                    .split(SAMPLE_SEPARATOR)
                    .map(ToString::to_string)
                    .collect();
                let mut path = path.iter();
                match self.0.path(&mut path) {
                    Some(FileSystemTree::File(content)) => Info::from((len(content), Vec::new())),
                    Some(FileSystemTree::Directory(content)) => Info::from((
                        SAMPLE_DIR_SIZE,
                        content.keys().map(ToString::to_string).collect(),
                    )),
                    None => panic!("Path does not exist"),
                }
            },
            join_path: |prefix, name| format!("{}{}{}", prefix, SAMPLE_SEPARATOR, name),
        })
    }
}

#[test]
fn flat() {
    let actual = SampleTree::create_sample().tree("flat");
    let expected = Tree {
        name: "flat".to_string(),
        data: len("") + len("a") + len("ab") + len("abc") + SAMPLE_DIR_SIZE,
        children: vec![
            Tree {
                name: "0".to_string(),
                data: len(""),
                children: Vec::new(),
            },
            Tree {
                name: "1".to_string(),
                data: len("a"),
                children: Vec::new(),
            },
            Tree {
                name: "2".to_string(),
                data: len("ab"),
                children: Vec::new(),
            },
            Tree {
                name: "3".to_string(),
                data: len("abc"),
                children: Vec::new(),
            },
        ],
    };
    assert_eq!(actual, expected);
}

#[test]
fn nested() {
    let actual = SampleTree::create_sample().tree("nested");
    let expected = Tree {
        name: "nested".to_string(),
        data: len("abcdef") + SAMPLE_DIR_SIZE + SAMPLE_DIR_SIZE,
        children: vec![Tree {
            name: "0".to_string(),
            data: len("abcdef") + SAMPLE_DIR_SIZE,
            children: vec![Tree {
                name: "1".to_string(),
                data: len("abcdef"),
                children: Vec::new(),
            }],
        }],
    };
    assert_eq!(actual, expected);
}

#[test]
fn empty_dir() {
    let actual = SampleTree::create_sample().tree("empty-dir");
    let expected = Tree {
        name: "empty-dir".to_string(),
        data: SAMPLE_DIR_SIZE,
        children: Vec::new(),
    };
    assert_eq!(actual, expected);
}
