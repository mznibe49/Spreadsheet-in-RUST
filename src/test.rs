//#[cfg(test)]
/*mod tests {
    use crate::spreadsheet;
    use std::collections::HashMap;

    #[test]
    fn test_check_extension(){
        let target = String::from("/target/debug/need_a_name");
        let data = String::from("data.csv");
        let view = String::from("view.csv");
        let user = String::from("user.txt");
        let changes = String::from("changes.txt");

        let v1 = vec![&target, &data, &user, &view, &changes];
        let v2 = vec![&target, &data, &view, &user, &changes];
        let v3 = vec![&target, &user, &changes, &data, &view];

        let t1 = (v1, true);
        let t2 = (v2, false);
        let t3 = (v3, false);

        let mut tests_vector = vec![t1, t2, t3];

        for test in &mut tests_vector {
            let result: std::io::Result<bool> = spreadsheet::check_extension(&mut test.0);
            match result {
                Ok(value) => assert_eq!(value, test.1),
                Err(_) => (),
            }
        }
    }

    #[test]
    fn test_check_formula() {
        let t1 = (String::from("=#(7, 2, 3, 0, 7)"), true);
        let t2 = (String::from("=#(2, 1, 13, abd, 23)"), false);
        let t3 = (String::from("=#(2, 2, 13, 1, 23)d"), false);
        let t4 = (String::from("=#(8999, 2, 130, 1, 23)"), true);
        let t5 = (String::from("=#(2, 2, 90, 1, 256)"), false);
        let t6 = (String::from("=#(2, 2, -1, 1, 23)"), false);

        let tests_vector: Vec<(String, bool)> = vec![t1, t2, t3, t4, t5, t6];

        for test in &tests_vector {
            let result: std::io::Result<bool> = spreadsheet::OccurCell::check_formula(&test.0);
            match result {
                Ok(value) => assert_eq!(value, test.1),
                Err(_) => (),
            }
        }
    }

    #[test]
    fn test_check_data_file() {
        let t1 = (String::from("testfiles/data/g0-data.csv"), true);
        let t2 = (String::from("testfiles/data/g1-data.csv"), true);
        let t3 = (String::from("testfiles/data/b0-data.csv"), false);
        let t4 = (String::from("testfiles/data/b1-data.csv"), false);
        let t5 = (String::from("testfiles/data/b2-data.csv"), false);

        let tests_vector: Vec<(String, bool)> = vec![t1, t2, t3, t4, t5];

        for test in &tests_vector {
            let result: std::io::Result<bool> = spreadsheet::check_data_file(&test.0);
            match result {
                Ok(value) => assert_eq!(value, test.1),
                Err(_) => (),
            }
        }
    }

    #[test]
    fn test_parse_formula() {
        let str = String::from("=#(2,2,  3, 3, 9)");
        let mut celio = spreadsheet::OccurCell {
            crd: (1, 1),
            occur: 0,
            c1: (0, 0),
            c2: (2, 2),
            value: 0,
            childs: Vec::new(),
            parents: Vec::new(),
        };

        let celio2 = spreadsheet::OccurCell {
            crd: (1, 1),
            occur: 0,
            c1: (2,2),
            c2: (3,3),
            value: 9,
            childs : Vec::new(),
            parents: Vec::new(),
        };
        celio.parse_formula(&str);
        assert_eq!(celio,celio2);
    }

    #[test]
    fn test_check_user_file(){
        let t1 = (String::from("testfiles/user/g0-user.txt"), true);
        let t2 = (String::from("testfiles/user/g1-user.txt"), true);
        let t3 = (String::from("testfiles/user/b0-user.txt"), false);
        let t4 = (String::from("testfiles/user/b1-user.txt"), false);
        let t5 = (String::from("testfiles/user/b2-user.txt"), false);
        let t6 = (String::from("testfiles/user/b3-user.txt"), false);
        let t7 = (String::from("testfiles/user/b4-user.txt"), false);
        let t8 = (String::from("testfiles/user/b5-user.txt"), false);

        let tests_vector: Vec<(String, bool)> = vec![t1, t2, t3, t4, t5, t6, t7, t8];

        for test in tests_vector {
            let result: std::io::Result<bool> = spreadsheet::check_user_file(&test.0);
            match result {
                Ok(value) => assert_eq!(value, test.1),
                Err(_) => (),
            }
        }
    }

    #[test]
    fn test_count_occurences() {
        /*
                0   1   2
            +   -   -   -   +   oc1 =#(0, 1, 2, 2, 1) = 3
        0   |   2   3   1   |   oc2 =#(1, 0, 2, 1, 0) = 1
        1   |   0   1   3   |   oc3 =#(0, 0, 2, 2, 3) = 2
        2   |   2   1   4   |
        3   |   oc1 oc2 oc3 |
            +   -   -   -   +
                                */

        let mut oc1 = spreadsheet::OccurCell {
                crd: (3, 0),
                occur: 3,
                c1: (0, 1),
                c2: (2, 2),
                value: 1,
                childs: Vec::new(),
                parents: Vec::new()
        };

        let mut oc2 = spreadsheet::OccurCell {
                crd: (3, 1),
                occur: 1,
                c1: (1, 0),
                c2: (2, 1),
                value: 0,
                childs: Vec::new(),
                parents: Vec::new()
        };

        let mut oc3 = spreadsheet::OccurCell {
                crd: (3, 2),
                occur: 2,
                c1: (0, 0),
                c2: (2, 2),
                value: 3,
                childs: Vec::new(),
                parents: Vec::new()
        };

        let mut s: spreadsheet::SpreadSheet = spreadsheet::SpreadSheet {
            r_max: 2,
            c_max: 3,
            cells: HashMap::new(),
            evals: vec![(3, 0), (3, 1), (3, 2)],
            changes: Vec::new(),
            errors: Vec::new()
        };

        s.cells.insert(
            (0, 0),
            spreadsheet::Cell::StaticCell(spreadsheet::StaticCell { crd: (0, 0), value: 2, parents: Vec::new() })
        );

        s.cells.insert(
            (0, 1),
            spreadsheet::Cell::StaticCell(spreadsheet::StaticCell { crd: (0, 1), value: 3, parents: Vec::new() })
        );

        s.cells.insert(
            (0, 2),
            spreadsheet::Cell::StaticCell(spreadsheet::StaticCell { crd: (0, 2), value: 1, parents: Vec::new() })
        );

        s.cells.insert(
            (1, 0),
            spreadsheet::Cell::StaticCell(spreadsheet::StaticCell { crd: (1, 0), value: 0, parents: Vec::new() })
        );

        s.cells.insert(
            (1, 1),
            spreadsheet::Cell::StaticCell(spreadsheet::StaticCell { crd: (1, 1), value: 1, parents: Vec::new() })
        );

        s.cells.insert(
            (1, 2),
            spreadsheet::Cell::StaticCell(spreadsheet::StaticCell { crd: (1, 2), value: 3, parents: Vec::new() })
        );

        s.cells.insert(
            (2, 0),
            spreadsheet::Cell::StaticCell(spreadsheet::StaticCell { crd: (2, 0), value: 2, parents: Vec::new() })
        );

        s.cells.insert(
            (2, 1),
            spreadsheet::Cell::StaticCell(spreadsheet::StaticCell { crd: (2, 1), value: 1, parents: Vec::new() })
        );

        s.cells.insert(
            (2, 2),
            spreadsheet::Cell::StaticCell(spreadsheet::StaticCell { crd: (2, 2), value: 4, parents: Vec::new() })
        );

        s.cells.insert(
            (3, 0),
            spreadsheet::Cell::OccurCell(oc1.clone())
        );

        s.cells.insert(
            (3, 1),
            spreadsheet::Cell::OccurCell(oc2.clone())
        );

        s.cells.insert(
            (3, 2),
            spreadsheet::Cell::OccurCell(oc3.clone())
        );

        let t1 = (&oc1, 3);
        let t2 = (&oc2, 1);
        let t3 = (&oc3, 2);

        let tests_vector = vec![t1, t2, t3];

        for test in &tests_vector {
            let result: std::io::Result<u32> = spreadsheet::SpreadSheet::count_occurences(&s, &test.0);
            match result {
                Ok(value) => assert_eq!(value, test.1),
                Err(_) => (),
            }
        }
    }

    #[test]
    fn test_check_view_file(){
        let file_name = String::from("../marvin/res/output/view0.csv");
        let result : std::io::Result<bool> = spreadsheet::check_view_file(&file_name);
        match result {
            Ok(value) =>  assert_eq!(value, true),
            Err(_) => (),
        }
    }
}

    #[test]
    fn test_count_occurence(){
        //let formula_value = String::from("=#(2, 1, 0, 1, 3)");
        let formula_position =  (2,2);
        let first_cell = (2,1);
        let sec_cell = (0,1);
        let dep = Vec::new();

        let dynamic_cell = DynamicCell {
            coordinates : formula_position,
            value : 3,
            cell_1 : first_cell,
            cell_2 : sec_cell,
            dependances : dep,
        };



    }
*/