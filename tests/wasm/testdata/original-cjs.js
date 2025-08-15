module.exports = class Up {
	constructor() {
		console.log('constructor')
	}

	fetch() {
		console.log('fetch')
	}

  cb(args, callback) {
    callback(null, args)
  }
}

