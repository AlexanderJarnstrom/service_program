# Service Program
Microservice documentation: [doc](./doc)

## Assignment description

### TODO

**The goal of this assignment is for you to create a microservice-based application according to the following principles**:

- [x] The application must be deployable using Kubernetes.
- [x] The application must consist of at least two different types of microservices and a database.
- [x] Each microservice must implement a REST API for access
- [x] The application must be accessible from “the outside” of kubernetes (e.g. via a web browser)
- [x] All microservices in the application must be horizontally scalable independently from each other.
- [x] Any microservice images required to run your application must have been pushed to docker hub so that Kubernetes may find them.
- [x] The application must make use of some form of database running as a separate microservice.
- [x] The database must use storage which is persistent across restarts of the deployment infrastructure.
- [x] The database does not have to be scalable.

**The following items shall be delivered**:

- [ ] A description of the software you have built and what it does.
- [ ] A software architecture design of your application, describing the role of each component in your system, their responsibilities, and the architecture principles (e.g. cloud architecture patterns) that are used to connect them to a functioning system. This also includes a mapping between software components and the microservices designed and built to implement the components.
- [ ] A discussion of the benefits and challenges with your architecture design. This must include a discussion about security. It must also include a discussion of what you have done or what can be done to mitigate the identified challenges.
- [ ] A link to a configuration management repository (e.g. git) where the source code of the application can be viewed. This must also include the code for your Kubernetes deployment.
- [ ] A video recording (5-10 minutes long) where you go through what the project is about, what the separate microservices do, run the project (using kubernetes), and access it through its user interface (e.g. a web browser), show any log output, and briefly walk through your kubernetes yaml file. You may upload this video or provide a link to it.

### Sides

**Be prepared to answer questions about**:

- Your software application idea
- The architecture design decisions in your application
- The business implications of these architecture decisions
- Interaction between different microservices
- Details of your deployment
- Security issues identified and/or mitigated

**Your application**:

- must display your ability to progamatically connect to and use a REST API.
- must display your ability to program a microservice that provides a REST API.
- may implement any software idea; it may also re-use the idea from an already existing software.
- may be too small or trivial to actually warrant scaling; please acknowledge this and provide instructions for what needs to be pretended for the application to make sense.
- may use any programming language.
- may use a different access interface than a web browser, as long as no additional tools need to be installed on my computer.
- may use any type of database (e.g. NoSQL, SQL, Graph, etc.).
- may be deployed to a cloud provider already.
