let is_e2e = variable::get("end2end");

if !is_e2e {
    file::delete("end-to-end");
}
