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

var mapview = Vue.component('mapview', {
  props: {
    content: Float64Array
  },
  data: function () {
    return {
      mymap: null
    }
  },
  watch: {
    content: function(val){
      console.log(val)
      Leaflet.marker([val[0], val[1]]).addTo(this.mymap)
      Leaflet.circle([val[0], val[1]], {
        color: 'red',
        fillColor: '#f03',
        fillOpacity: 0.5,
        radius: 10
      }).addTo(this.mymap);

      this.mymap.setView([val[0], val[1]], 24)
    }
  },
  mounted () {
    this.mymap = Leaflet.map('mapid').setView([-14.2350, -51.9253], 5);
    console.log(this.content)
    Leaflet.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
	    maxZoom: 19
    }).addTo(this.mymap);
  },
  template: '<div id="mapid"></div>'
})

const vm = new Vue({
  el: '#app',
  data: {
    name: '',
    type: '',
    size: '',
    content: ''
  },
  components: {
    'mapview': mapview
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
