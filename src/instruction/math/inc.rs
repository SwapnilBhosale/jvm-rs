use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn IINC(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IINC");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (index, code_reader) = code_reader.read_u8();
    let (val_1, code_reader) = code_reader.read_u8();
    let index = index as usize;
    let val_1 = val_1 as i32;

    let val_2 = local_vars.get_int(index);
    let val = val_1 + val_2;
    let local_vars = local_vars.set_int(index, val);

    let frame = Frame {
        operand_stack,
        local_vars,
    };
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}
