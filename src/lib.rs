pub mod common;
pub mod pak;

#[cfg(test)]
mod tests {
    #[cfg(feature = "revpk")]
    use crate::pak::revpk::format::{
        VPKDirectoryEntryRespawn, VPKFilePartEntryRespawn, VPKRespawn,
    };
    use crate::{
        common::format::VPKDirectoryEntry,
        pak::{v1::format::VPKVersion1, v2::format::VPKVersion2},
    };
    use std::{fs::File, io::Seek, path::Path};

    #[test]
    fn read_empty_vpk_v1() {
        let path = Path::new("./test_files/empty_v1_dir.vpk");
        let mut file = File::open(path).expect("Failed to open file");
        let vpk = VPKVersion1::from(&mut file);
        assert_eq!(vpk.tree.files.len(), 0, "VPK tree should have 0 entries");
        assert_eq!(
            vpk.tree.files.get("test/file.txt"),
            None,
            "File \"test/file.txt\" shouldn't exist"
        );
        assert!(
            file.stream_position().unwrap() >= file.seek(std::io::SeekFrom::End(0)).unwrap() - 1,
            "Should be at end of file"
        );
    }

    #[test]
    fn read_single_file_vpk_v1() {
        let path = Path::new("./test_files/single_file_v1_dir.vpk");
        let mut file = File::open(path).expect("Failed to open file");
        let vpk = VPKVersion1::from(&mut file);
        assert_eq!(vpk.tree.files.len(), 1, "VPK tree should have 1 entry");
        assert_eq!(
            vpk.tree.files.get("test/file.txt"),
            Some(&VPKDirectoryEntry::new()),
            "File \"test/file.txt\" should exist"
        );
        assert!(
            file.stream_position().unwrap() >= file.seek(std::io::SeekFrom::End(0)).unwrap() - 1,
            "Should be at end of file"
        );
    }

    #[test]
    fn read_big_vpk_v1() {
        let path = Path::new("./test_files/portal/pak01_dir.vpk");
        let mut file: File = File::open(path).expect("Failed to open file");
        let vpk = VPKVersion1::from(&mut file);
        assert_eq!(
            vpk.tree.files.len(),
            449,
            "VPK tree should have 449 entries"
        );
        assert!(
            file.stream_position().unwrap() >= file.seek(std::io::SeekFrom::End(0)).unwrap() - 1,
            "Should be at end of file"
        );
    }

    #[test]
    fn read_single_file_vpk_v2() {
        let path = Path::new("./test_files/single_file_v2_dir.vpk");
        let mut file: File = File::open(path).expect("Failed to open file");
        let vpk = VPKVersion2::from(&mut file);
        assert_eq!(
            vpk.tree.files.len(),
            1,
            "VPK tree should have 1 entry"
        );
        assert!(
            file.stream_position().unwrap() >= file.seek(std::io::SeekFrom::End(0)).unwrap() - 1,
            "Should be at end of file"
        );
    }

    #[test]
    fn read_big_vpk_v2() {
        let path = Path::new("./test_files/tf2/tf2_sound_misc_dir.vpk");
        let mut file: File = File::open(path).expect("Failed to open file");
        let vpk = VPKVersion2::from(&mut file);
        assert_eq!(
            vpk.tree.files.len(),
            3230,
            "VPK tree should have 3230 entries"
        );
        assert!(
            file.stream_position().unwrap() >= file.seek(std::io::SeekFrom::End(0)).unwrap() - 1,
            "Should be at end of file"
        );
    }

    #[cfg(feature = "revpk")]
    #[test]
    fn read_single_file_vpk_revpk() {
        let path = Path::new("./test_files/single_file_revpk_dir.vpk");
        let mut file = File::open(path).expect("Failed to open file");
        let vpk = VPKRespawn::from(&mut file);

        assert_eq!(vpk.tree.files.len(), 1, "VPK tree should have 1 entry");

        let mut dir_entry = VPKDirectoryEntryRespawn::new();
        dir_entry.file_parts.push(VPKFilePartEntryRespawn::new());
        assert_eq!(
            vpk.tree.files.get("test/file.txt"),
            Some(&dir_entry),
            "File \"test/file.txt\" should exist"
        );
        assert!(
            file.stream_position().unwrap() >= file.seek(std::io::SeekFrom::End(0)).unwrap() - 1,
            "Should be at end of file"
        );
    }

    #[cfg(feature = "revpk")]
    #[test]
    fn read_big_vpk_revpk() {
        let path = Path::new("./test_files/titanfall/englishclient_mp_colony.bsp.pak000_dir.vpk");
        let mut file = File::open(path).expect("Failed to open file");
        let vpk = VPKRespawn::from(&mut file);
        assert_eq!(
            vpk.tree.files.len(),
            5723,
            "VPK tree should have 5723 entries"
        );
        assert!(
            file.stream_position().unwrap() >= file.seek(std::io::SeekFrom::End(0)).unwrap() - 1,
            "Should be at end of file"
        );
    }
}
