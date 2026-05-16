export function studentListToText(studentList = []) {
  return studentList.map(student => student.name).join('\n')
}
