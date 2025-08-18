pipeline {
    agent any

    environment {
        DOCKERHUB_USERNAME = "esteban1930"
        APP_NAME = "backend-1"
        DOCKER_CREDENTIALS_ID = "dockerhub-credentials"
        IMAGE_TAG = ""
    }

    stages {
        stage('Determine Image Tag') {
            steps {
                script {
                    if (env.TAG_NAME) {

                        env.IMAGE_TAG = env.TAG_NAME.replaceFirst('^v', '')
                        echo "Build iniciado por Git tag. Se usará la etiqueta de imagen: ${env.IMAGE_TAG}"
                    } else {
                        def shortCommit = sh(script: 'git rev-parse --short HEAD', returnStdout: true).trim()

                        def sanitizedBranchName = env.BRANCH_NAME.replace('/', '-')
                        env.IMAGE_TAG = "${sanitizedBranchName}-${shortCommit}"
                        echo "Build iniciado por una rama. Se usará la etiqueta de desarrollo: ${env.IMAGE_TAG}"
                    }
                }
            }
        }

        stage('Build, Test, and Push Image') {
            steps {
                script {
                    echo "Iniciando build de Docker con la etiqueta: ${env.IMAGE_TAG}..."
                    echo "Esto compilará, probará y empaquetará la aplicación Rust."
                    
                    def imageName = "${env.DOCKERHUB_USERNAME}/${env.APP_NAME}:${env.IMAGE_TAG}"
                    def customImage

                    try {
                        customImage = docker.build(imageName)

                    } catch (e) {
                        echo "El build de Docker falló. Revisa los logs por errores de compilación o test. ${e.message}"
                        error "Build fallido."
                    }

                    echo "Build y tests exitosos. Empujando la imagen a Docker Hub..."
                    docker.withRegistry("https://index.docker.io/v1/", env.DOCKER_CREDENTIALS_ID) {
                        customImage.push()
                    }
                }
            }
        }
    }

    post {
        always {
            echo "Pipeline finalizado."
            cleanWs()
        }
    }
}
