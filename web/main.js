'use strict'

const input = document.getElementById('file')
const type = document.getElementById('type')
const size = document.getElementById('size')
const name = document.getElementById('name')
const content = document.getElementById('content')

if (input.files[0]) {
  update(input.files[0]).catch(err => console.error(err))
}

input.addEventListener('change', ev => {
  const file = ev.target.files[0]
  update(file).catch(err => console.error(err))
})

async function update(file) {
  name.innerText = file.name
  type.innerText = file.type
  size.innerText = file.size
  content.innerText = await readFile(file)
}

function readContent(file) {
  return new Promise((resolve, reject) => {
    let reader = new FileReader()
    reader.onload = resolve
    reader.onerror = reject
    reader.readAsText(file)
  })
}

async function readFile(file) {
  let content = await readContent(file)
  return content.target.result
}
