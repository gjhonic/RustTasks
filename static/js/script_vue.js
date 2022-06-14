let app = new Vue({
    el: '#app',
    data: {
        config: {
            projectName: 'RustTasks',
            version: '0.1',
            lang: 'en',
            author: 'gjhonic',
        },
        //Ссылки
        url_get_tasks: '/get-tasks',
        //Данные
        tasksData: [],
        property: '',
    },
    computed: {
        Tasks() {
            return this.tasksData;
        },
        Branches() {
            return this.branchesData;
        },
    },
    methods: {
        loadTasks(callback) {
            fetch(this.url_get_tasks, {
                method: "GET",
            }).then(response => response.json()).then(data => {
                console.log(this.data);

                if (typeof callback !== 'undefined') {
                    let timerId = setTimeout(function () {
                        callback();
                    }, 500);
                }
            });
        },
    },
    mounted() {
        this.loadTasks();
    },
});
