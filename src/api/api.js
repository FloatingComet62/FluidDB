const express = require('express');
const app = express();
const port = 3000;
const database = require('../database/high');

app.listen(port, ()=> console.log(`API running on http://127.0.0.1:${port}`));

app.get('/', (_, response)=>{
    response.status(200).send({
        data: database.getAll(),
        error: null
    });
});

app.get('/reset', (_, response)=>{
    database.reset();
    response.status(200).send({
        data: 'Done',
        error: null
    })
});

app.use('/ocean/:name', (request, response)=>{
    const name = request.params.name;
    let output = { code: 400, data: { data: null, error: `No ocean with name ${name}` } };
    if(request.method === 'GET'){
        const data = database.getOcean(name);
        if(data){
            output = { code: 200, data: {
                data: data,
                error: null }
            };
        }
    }else if(request.method === 'POST'){
        database.addOcean(name);
        output = { code: 200, data: {
            data: database.getOcean(name),
            error: null }
        };
    }

    response.status(output.code).send(output.data);
});

app.use('/sea/:oceanname/:name', (request, response)=>{
    const name = request.params.name;
    const oceanname = request.params.oceanname;
    let output = { code: 400, data: { data: null, error: `No sea with name ${name}` } };
    if(request.method === 'GET'){
        const data = database.getSea(oceanname, name);
        if(data){
            output = { code: 200, data: {
                data: data,
                error: null }
            }
        }
    }else if(request.method === 'POST'){
        database.addSea(oceanname, name);
        output = { code: 200, data: {
            data: database.getSea(oceanname, name),
            error: null }
        }
    }

    response.status(output.code).send(output.data);
});

app.use('/river/:oceanname/:seaname/:name', (request, response)=>{
    const name = request.params.name;
    const oceanname = request.params.oceanname;
    const seaname = request.params.seaname;
    let output  = { code: 400, data: { data: null, error: `No river with name ${name}` } };
    if(request.method === 'GET'){
        const data = database.getRiver(oceanname, seaname, name);
        if(data){
            output = { code: 200, data: {
                data: data,
                error: null }
            }
        }
    }else if(request.method === 'POST'){
        database.addRiver(oceanname, seaname, name);
        output = { code: 200, data: {
            data: database.getRiver(oceanname, seaname, name),
            error: null }
        }
    }

    response.status(output.code).send(output.data);
});