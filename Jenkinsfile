pipeline {
    agent any

    environment {
        DOCKERHUB_USERNAME = "esteban1930"
        APP_NAME = "backend-1"
        DOCKER_CREDENTIALS_ID = "dockerhub-credentials"

        HELM_CHART_BRANCH = "master"
        GIT_CREDENTIALS_ID = "github-credentials"
        HELM_CHART_REPO = "https://github.com/EstebanForero/parcial-1"
    }

    stages {
        stage('Build, Test, and Push Image') {
            steps {
                script {
                    echo "Starting Docker build..."
                    echo "This will compile, test, and package the Rust application."

                    def imageName = "${env.DOCKERHUB_USERNAME}/${env.APP_NAME}:1.${env.BUILD_NUMBER}.0"
                    def customImage

                    try {
                        customImage = docker.build(imageName)

                    } catch (e) {
                        // This block catches the error if the build fails (e.g., test failure)
                        echo "Docker build failed. Check the logs for test failures or compilation errors. ${e.message}"
                        error "Build failed."
                    }

                    echo "Build and tests succeeded. Pushing image to Docker Hub..."
                    docker.withRegistry("https://index.docker.io/v1/", env.DOCKER_CREDENTIALS_ID) {
                        customImage.push()
                    }
                }
            }
        }

        stage('Update Helm Chart pipeline') {
            steps {
                script {
                    sh 'rm -rf chart'
                    sh "git clone --branch ${env.HELM_CHART_BRANCH} https://github.com/EstebanForero/parcial-1.git chart"

                    dir('chart') {
                        if (true) {
                            sh """
                                sed -i '/^backend:/,/^[^ ]/{/tag:/s/tag:[ ]*.*/tag: "1.${BUILD_NUMBER}.0"/}' values.yaml
                                """
                        }
                        if (false) {
                            sh """
                                sed -i '/^frontend:/,/^[^ ]/{/tag:/s/tag:[ ]*.*/tag: "1.${BUILD_NUMBER}.0"/}' values.yaml
                                """
                        }

                        sh "git config --local user.email 'jenkins@example.com'"
                        sh "git config --local user.name 'Jenkins'"
                        sh "git add ."
                        sh "git commit -m 'Bump version and update image tag(s)'"
                        sh "git push origin ${env.HELM_CHART_BRANCH}"
                    }
                }
            }
        }
    }

    post {
        always {
            echo "Pipeline finished."
            cleanWs()
        }
    }
}
