import Vue from 'vue/dist/vue.esm'
import Leaflet from 'leaflet'
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
  data: {
    name: '',
    type: '',
    size: '',
    content: ''
  },
  methods: {
    change({ target: { files } }) {
      this.name = files[0].name
      this.type = files[0].type
      this.size = files[0].size
      this.updateContent(files[0])
    },
    async updateContent(file) {
      let buffer = await readFile(file)
      this.content = (await rust).read_metadata(new Uint8Array(buffer))
    }
  }
})

Vue.component('mapView', {
  props: {
    latitude: Number,
    longitude: Number
  },
  data: function () {
    return {
      mymap: null
    }
  },
  mounted () {
    this.mymap = Leaflet.map('mapid').setView([51.505, -0.09], 13);
  },
  template: '<div id="mapid"></div>'
})
