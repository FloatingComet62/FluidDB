const Sea = require('./sea');

class Ocean{
    constructor(database, name){
        this.name = name;
        this.database = database;
        this.database.addOcean(this.name);
    }
    createSea(name){
        return new Sea(this.database, this.name, name);
    }
}

module.exports = Ocean;