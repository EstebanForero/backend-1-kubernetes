pipeline {
    agent any

    environment {
        DOCKERHUB_USERNAME = "esteban1930"

        APP_NAME = "backend-1"

        DOCKER_CREDENTIALS_ID = "dockerhub-credentials"
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
    }

    post {
        always {
            echo "Pipeline finished."
            cleanWs()
        }
    }
}
