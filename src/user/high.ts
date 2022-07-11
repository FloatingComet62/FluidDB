import low from './low'

function addUser(username: string, password: string, rootPassword: string) {
    const { success, message } = low.getData()
    if (!success) return { success: false, message }
    if (!(message.root === rootPassword)) return { success: false, message: 'Incorrect root password' }
    if (username==='') return { success: false, message: 'Username must not be empty' }
    if (username==='root') return { success: false, message: 'Username must not be root' }
    if (message[username]) return { success: false, message: 'User already exists' }

    message[username] = password
    low.setData(message)
    return { success: true, message: 'User added' }
}

function deleteUser(username: string, rootpassword: string) {
    const { success, message } = low.getData()
    if (!success) return { success: false, message }
    if (!(message.root === rootpassword)) return { success: false, message: 'Incorrect root password' }
    if (username==='') return { success: false, message: 'Username must not be empty' }
    if (username==='root') return { success: false, message: 'Username must not be root' }
    if (!message[username]) return { success: false, message: 'User does not exist' }

    delete message[username]
    low.setData(message)
    return { success: true, message: 'User deleted' }
}

export default {
    addUser,
    deleteUser,
}