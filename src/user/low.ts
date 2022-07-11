
import fs from 'fs'
import { userResponse } from '../types'



function getData(): userResponse {
    try {
        const raw = fs.readFileSync(
            'user.json',
            'utf8'
        )
        try { return { success: true, message: JSON.parse(raw) } }
        catch { return { success: false, message: 'Error parsing JSON' } }
    } catch { return { success: false, message: 'Error getting data' } }
}
function setData(data: Object) {
    try {
        const parsed = JSON.stringify(data)
        try {
            fs.writeFileSync(
                'user.json',
                parsed,
                { encoding: 'utf-8' }
            )
            return { success: true, message: 'Data saved' }
        } catch { return { success: false, message: 'Error saving data' } }
    } catch { return { success: false, message: 'Error parsing data' } }
}


export default {
    getData,
    setData
}