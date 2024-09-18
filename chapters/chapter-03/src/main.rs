mod a_raw_syscall;
mod b_normal_syscall;
mod the_rust_way;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    a_raw_syscall::syscall("Hello world from raw syscall!\n".into());
    b_normal_syscall::syscall("Hello world from normal syscall!\n".into())?;
    the_rust_way::syscall("Hello world from rust!".into());

    Ok(())
}
