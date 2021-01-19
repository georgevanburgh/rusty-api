module.exports = {
    devServer: {
        proxy: {
            '/has_session': 'http://localhost:8081',
            '/create_session': 'http://localhost:8081'
        }
    }
};