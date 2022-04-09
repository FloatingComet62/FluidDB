const express = require('express');
const app = express();
const port = 3000;
const database = require('../database/high');

app.listen(port, ()=> console.log(`API running on http://127.0.0.1:${port}`));

app.get('/reset', (_, response)=>{
    database.reset();
    response.status(200).send({
        data: 'Done',
        error: null
    })
});

app.use('/:oceanname?/:seaname?/:rivername?/:wavename?/:value?', (request, response)=>{
    const oceanname = request.params.oceanname;
    const seaname = request.params.seaname;
    const rivername  = request.params.rivername;
    const wavename = request.params.wavename;
    const value = request.params.value;
    const isGET = request.method === 'GET';
    const isPOST = request.method === 'POST';
    const isDELETE = request.method === 'DELETE';
    let output  = { code: 400, data: { data: null }};

    // /
    if(!oceanname && !seaname && !rivername && !wavename && !value){
        output.code = 200;
        output.data = database.getAll();
    
    // /:oceanname
    }else if(oceanname && !seaname && !rivername && !wavename && !value){
        output.code = 200;
        if(isGET){
            output.data = database.getOcean(oceanname);
        }else if(isPOST){
            database.addOcean(oceanname);
            output.data = database.getOcean(oceanname);
        }else if(isDELETE){
            database.removeOcean(oceanname);
            output.data = 'Done';
        }

    // /:oceanname/:seaname
    }else if(oceanname && seaname && !rivername && !wavename && !value){
        output.code = 200;
        if(isGET){
            output.data = database.getSea(oceanname, seaname);
        }else if(isPOST){
            database.addSea(oceanname, seaname);
            output.data = database.getSea(oceanname, seaname);
        }else if(isDELETE){
            database.removeSea(oceanname, seaname);
            output.data = 'Done';
        }

    // /:oceanname/:seaname/:rivername
    }else if(oceanname && seaname && rivername && !wavename && !value){
        output.code = 200;
        if(isGET){
            output.data = database.getRiver(oceanname, seaname, rivername);
        }else if(isPOST){
            database.addRiver(oceanname, seaname, rivername);
            output.data = database.getRiver(oceanname, seaname, rivername);
        }else if(isDELETE){
            database.removeRiver(oceanname, seaname, rivername);
            output.data = 'Done';
        }
    
    // /:oceanname/:seaname/:rivername/:wavename
    }else if(oceanname && seaname && rivername && wavename){
        if(isGET){
            output.code = 200;
            output.data = database.getRiver(oceanname, seaname, rivername)[wavename];
        
    // /:oceanname/:seaname/:rivername/:wavename/:value        
        }else if(isPOST){
            if(value){
                output.code = 200;
                database.addWave(oceanname, seaname, rivername, wavename, value);
                output.data = database.getWave(oceanname, seaname, rivername, wavename);
            }
        }else if(isDELETE){
            output.code = 200;
            database.removeWave(oceanname, seaname, rivername, wavename);
            output.data = 'Done';
        }
    }else{
        output.code = 404;
    }

    response.status(output.code).send(output.data);
})