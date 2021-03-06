use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn NOP(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}
