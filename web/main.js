import Vue from 'vue/dist/vue.esm'
const rust = import('../bindgen/lp.js')

function readFile(file) {
  return new Promise((resolve, reject) => {
    let reader = new FileReader()
    reader.onload = ev => resolve(ev.target.result)
    reader.onerror = reject
    reader.readAsArrayBuffer(file)
  })
}

const vm = new Vue({
  el: '#app',
  data: { name: '', type: '', size: '', content: '' },
  methods: {
    change({ target: { files } }) {
      this.name = files[0].name
      this.type = files[0].type
      this.size = files[0].size
      this.updateContent(files[0])
    },
    async updateContent(file) {
      let buffer = await readFile(file)
      this.content = (await rust).read_file(new Uint8Array(buffer))
    }
  }
})
